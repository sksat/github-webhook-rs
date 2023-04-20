pub mod name_types;

use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::ir::{
    LiteralKeyMap, RustContainerAttrs, RustEnum, RustEnumMemberKind, RustFieldAttr, RustFieldAttrs,
    RustMemberType, RustSegment, RustStruct, RustStructMember, RustType, SerdeFieldAttr, TypeName,
};

pub fn interface2struct<'input>(
    st: &mut FrontendState<'input, '_>,
    interface: &'input swc_ecma_ast::TsInterfaceDecl,
    lkm: &mut LiteralKeyMap,
) {
    let name = interface.id.sym.to_string();
    let ibody = &interface.body.body;

    let mut rmember = Vec::new();

    for member in ibody {
        let prop = member.as_ts_property_signature().unwrap();
        let mut is_optional = prop.optional;
        //dbg!(prop);

        let mut pkey: &str = match &*prop.key {
            swc_ecma_ast::Expr::Ident(pkey) => &pkey.sym,
            swc_ecma_ast::Expr::Lit(swc_ecma_ast::Lit::Str(_pkey)) => {
                // TODO: use &pkey.value.as_ref()
                continue;
            }
            _ => unreachable!(),
        };
        //let pkey = if let Some(pkey) = &prop.key.as_ident() {
        //    // ident
        //    &pkey.sym
        //} else {
        //    // interface { "+1": number; "-1": number; }
        //    // TODO: parse
        //    dbg!(prop);
        //    "+1"
        //};

        let mut attr = RustFieldAttrs::Default;

        // avoid conflict to Rust reserved word
        static RENAME_RULES: Lazy<HashMap<&str, &str>> =
            Lazy::new(|| HashMap::from_iter([("type", "type_"), ("ref", "ref_")]));
        if let Some(renamed) = RENAME_RULES.get(pkey) {
            attr.add_attr(RustFieldAttr::Serde(SerdeFieldAttr::Rename(
                pkey.to_owned(),
            )));
            pkey = renamed;
        }

        let ptype = &prop.type_ann.as_ref().unwrap().type_ann;
        let (is_optional2, ty) = ts_type_to_rs(
            st,
            Some(TypeConvertContext {
                struct_name: &name,
                field_name: pkey,
            }),
            ptype,
        );
        is_optional |= is_optional2;

        fn extract_literal_type(ptype: &swc_ecma_ast::TsType) -> Option<&str> {
            ptype.as_ts_lit_type()?.lit.as_str()?.raw.as_deref()
        }

        if let Some(lit) = extract_literal_type(ptype) {
            lkm.entry(name.clone()).or_default().insert(
                pkey.to_owned(),
                lit.strip_prefix('\"')
                    .unwrap()
                    .strip_suffix('\"')
                    .unwrap()
                    .to_owned(),
            );
        }

        rmember.push(RustStructMember {
            ty: RustMemberType { ty, is_optional },
            name: pkey.to_string(),
            attr,
            comment: None,
        });
    }

    let s = RustStruct {
        attr: RustContainerAttrs::Default,
        name,
        is_borrowed: false,
        member: rmember,
    };
    st.segments.push(RustSegment::Struct(s));
}

pub fn tunion2enum(name: &str, tunion: &swc_ecma_ast::TsUnionType) -> RustEnum {
    let mut member = Vec::new();
    for t in &tunion.types {
        match &**t {
            swc_ecma_ast::TsType::TsTypeRef(tref) => {
                // export type Hoge = Fuga | Fuge;
                let i = &tref.type_name.as_ident().unwrap();
                let sym = i.sym.to_string();
                member.push(
                    RustEnumMemberKind::Unary(TypeName {
                        name: sym,
                        is_borrowed: false,
                    })
                    .into(),
                );
                //write!(out, "{}({})", &i.sym, &i.sym)?;
            }
            swc_ecma_ast::TsType::TsLitType(tlit) => {
                // export type BranchProtectionRuleEnforcementLevel =
                //   | "off" | "none_admins" | "everyone";
                let s = match tlit.lit {
                    swc_ecma_ast::TsLit::Str(ref s) => s.value.as_ref().to_string(),
                    _ => todo!(),
                };
                //write!(out, "{}", s)?;
                member.push(RustEnumMemberKind::Nullary(s).into());
            }
            swc_ecma_ast::TsType::TsArrayType(_tarray) => {
                // export WebhookEvents = | ( | "a" | "b" | "c" )[] | ["*"];
                // (| "a" | "b" | "c")[] <-
                //dbg!(tarray);
            }
            swc_ecma_ast::TsType::TsTupleType(_ttuple) => {
                // export WebhookEvents = | ( | "a" | "b" | "c" )[] | ["*"];
                // ["*"] <-
                //dbg!(ttuple);
            }
            swc_ecma_ast::TsType::TsTypeLit(_ttlit) => {
                // export PullRequestReviewRequestRemovedEvent =
                // | {
                //     action: "review_request_removed";
                //     requested_reviewer: User;
                //    }
                //  | {
                //     action: "review_request_removed";
                //     requested_reviewer: Team;
                //  };
                //dbg!(ttlit);
            }
            _ => {
                dbg!(t);
                unreachable!()
            }
        }
        //writeln!(out, ",")?;
    }

    RustEnum {
        attr: RustContainerAttrs::Default,
        name: name.to_string(),
        is_borrowed: false,
        member,
    }
}

pub struct FrontendState<'input, 'output> {
    pub segments: &'output mut Vec<RustSegment>,
    pub name_types: name_types::State<'input>,
}

fn ts_keyword_type_to_rs(typ: &swc_ecma_ast::TsKeywordType) -> RustType {
    use swc_ecma_ast::TsKeywordTypeKind;
    match typ.kind {
        TsKeywordTypeKind::TsStringKeyword => RustType::String { is_borrowed: false },
        TsKeywordTypeKind::TsNumberKeyword => RustType::Number,
        TsKeywordTypeKind::TsBooleanKeyword => RustType::Boolean,
        TsKeywordTypeKind::TsNullKeyword => RustType::Unit,
        _ => {
            unimplemented!("{:?}", typ.kind);
        }
    }
}

pub struct TypeConvertContext<'a> {
    struct_name: &'a str,
    field_name: &'a str,
}

pub fn ts_type_to_rs<'input>(
    st: &mut FrontendState<'input, '_>,
    ctxt: Option<TypeConvertContext>,
    typ: &'input swc_ecma_ast::TsType,
) -> (bool, RustType) {
    use swc_ecma_ast::TsKeywordTypeKind;
    use swc_ecma_ast::TsUnionOrIntersectionType;

    let mut nullable = false;

    let typ = match typ {
        swc_ecma_ast::TsType::TsKeywordType(tk) => {
            let t = ts_keyword_type_to_rs(tk);
            if let RustType::Unit = &t {
                nullable = true;
            }
            t
        }
        swc_ecma_ast::TsType::TsUnionOrIntersectionType(tsuoi) => {
            match tsuoi {
                TsUnionOrIntersectionType::TsUnionType(tunion) => {
                    // nullable check
                    let mut types: Vec<&Box<swc_ecma_ast::TsType>> = tunion.types.iter().collect();
                    // obtain non-null types within union, judging if there is null keyword
                    types.retain(|t| {
                        if let Some(tkey) = t.as_ts_keyword_type() {
                            if tkey.kind == TsKeywordTypeKind::TsNullKeyword {
                                nullable = true;
                                return false;
                            }
                        }
                        true
                    });

                    assert!(!types.is_empty());
                    if types.len() == 1 {
                        let (n, t) = ts_type_to_rs(st, ctxt, types[0]);
                        return (n || nullable, t);
                    }

                    // strings check: "Bot" | "User" | "Organization"
                    if types.iter().all(|t| {
                        t.as_ts_lit_type()
                            .map(|t| t.lit.is_str())
                            .unwrap_or_default()
                    }) {
                        let mut variants: Vec<&str> = types
                            .iter()
                            .map(|t| {
                                t.as_ts_lit_type()
                                    .unwrap()
                                    .lit
                                    .as_str()
                                    .unwrap()
                                    .value
                                    .as_ref()
                            })
                            .collect();
                        variants.sort();
                        let TypeConvertContext {
                            struct_name,
                            field_name,
                        } = ctxt.expect("provide ctxt");
                        let tn =
                            name_types::string_literal_union(st, variants, struct_name, field_name);
                        return (nullable, RustType::Custom(tn));
                        //TODO: comment strs  // {:?}", strs));
                    }

                    // other types
                    dbg!(types);

                    RustType::UnknownUnion
                }
                TsUnionOrIntersectionType::TsIntersectionType(_tints) => {
                    // dbg!(tints);
                    //todo!();
                    RustType::UnknownIntersection
                }
            }
        }
        swc_ecma_ast::TsType::TsLitType(_tslit) => RustType::UnknownLiteral,
        swc_ecma_ast::TsType::TsTypeRef(tref) => {
            let id = tref.type_name.as_ident().unwrap().sym.as_ref();
            RustType::Custom(TypeName {
                name: id.to_owned(),
                is_borrowed: false,
            })
        }
        swc_ecma_ast::TsType::TsArrayType(tarray) => {
            let (_n, etype) = ts_type_to_rs(st, ctxt, &tarray.elem_type);
            //format!("Vec<{etype}>")
            RustType::Array(Box::new(etype))
        }
        swc_ecma_ast::TsType::TsTypeLit(tlit) => {
            for _m in &tlit.members {
                // dbg!(m);
            }
            RustType::Unknown
        }
        _ => {
            //dbg!(typ);
            //todo!();
            RustType::Unknown
        }
    };

    (nullable, typ)
}
