pub mod name_types;

use once_cell::sync::Lazy;
use std::{borrow::Cow, collections::HashMap};

use crate::{
    case,
    ir::{
        LiteralKeyMap, RustAlias, RustContainerAttrs, RustEnum, RustEnumMemberKind, RustFieldAttr,
        RustFieldAttrs, RustMemberType, RustSegment, RustStruct, RustStructMember, RustType,
        SerdeFieldAttr, TypeName,
    },
};

pub fn interface2struct<'input>(
    st: &mut FrontendState<'input, '_>,
    interface: &'input swc_ecma_ast::TsInterfaceDecl,
    lkm: &mut LiteralKeyMap,
) {
    let name = interface.id.sym.as_ref();
    let ibody = &interface.body.body;

    let member = ibody
        .iter()
        .map(|m| m.as_ts_property_signature().unwrap())
        .flat_map(|prop| {
            ts_prop_signature(
                prop,
                st,
                TypeConvertContext {
                    path: Path::from_iter([Cow::Borrowed(name)]),
                },
                name,
                lkm,
            )
        })
        .collect();

    let name = name.to_owned();
    let s = RustStruct {
        attr: RustContainerAttrs::Default,
        name,
        is_borrowed: false,
        member,
    };
    st.segments.push(RustSegment::Struct(s));
}

pub fn ts_prop_signature<'input>(
    prop: &'input swc_ecma_ast::TsPropertySignature,
    st: &mut FrontendState<'input, '_>,
    mut ctxt: TypeConvertContext<'input>,
    name: &str,
    lkm: &mut HashMap<String, HashMap<String, String>>,
) -> Option<RustStructMember> {
    let mut is_optional = prop.optional;
    let mut pkey: &str = match &*prop.key {
        swc_ecma_ast::Expr::Ident(pkey) => &pkey.sym,
        swc_ecma_ast::Expr::Lit(swc_ecma_ast::Lit::Str(_pkey)) => {
            // TODO: use &pkey.value.as_ref()
            return None;
        }
        _ => unreachable!(),
    };
    let mut attr = RustFieldAttrs::Default;
    // avoid conflict to Rust reserved word
    static RENAME_RULES: Lazy<HashMap<&str, &str>> =
        Lazy::new(|| HashMap::from_iter([("type", "type_"), ("ref", "ref_"), ("self", "self_")]));
    if let Some(renamed) = RENAME_RULES.get(pkey) {
        attr.add_attr(RustFieldAttr::Serde(SerdeFieldAttr::Rename(
            pkey.to_owned(),
        )));
        pkey = renamed;
    }
    let ptype = &prop.type_ann.as_ref().unwrap().type_ann;
    ctxt.path.push(Cow::Borrowed(pkey));

    let (is_optional2, ty) = ts_type_to_rs(st, Some(ctxt), ptype, lkm);
    is_optional |= is_optional2;

    fn extract_literal_type(ptype: &swc_ecma_ast::TsType) -> Option<&str> {
        ptype.as_ts_lit_type()?.lit.as_str()?.raw.as_deref()
    }
    if let Some(lit) = extract_literal_type(ptype) {
        lkm.entry(name.to_owned()).or_default().insert(
            pkey.to_owned(),
            lit.strip_prefix('\"')
                .unwrap()
                .strip_suffix('\"')
                .unwrap()
                .to_owned(),
        );
    }
    Some(RustStructMember {
        ty: RustMemberType { ty, is_optional },
        name: pkey.to_string(),
        attr,
        comment: None,
    })
    //dbg!(prop);

    //let pkey = if let Some(pkey) = &prop.key.as_ident() {
    //    // ident
    //    &pkey.sym
    //} else {
    //    // interface { "+1": number; "-1": number; }
    //    // TODO: parse
    //    dbg!(prop);
    //    "+1"
    //};
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
        TsKeywordTypeKind::TsUnknownKeyword => RustType::Unknown,
        _ => {
            unimplemented!("{:?}", typ.kind);
        }
    }
}

pub type Path<'a> = Vec<Cow<'a, str>>;

#[derive(Clone)]
pub struct TypeConvertContext<'a> {
    pub path: Path<'a>,
}

pub fn ts_type_to_rs<'input>(
    st: &mut FrontendState<'input, '_>,
    ctxt: Option<TypeConvertContext<'input>>,
    typ: &'input swc_ecma_ast::TsType,
    lkm: &mut HashMap<String, HashMap<String, String>>,
) -> (bool, RustType) {
    use swc_ecma_ast::TsKeywordTypeKind;
    use swc_ecma_ast::TsUnionOrIntersectionType;

    let mut nullable = false;

    let typ = 't: {
        match typ {
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
                        let mut types: Vec<&Box<swc_ecma_ast::TsType>> =
                            tunion.types.iter().collect();
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
                            let (n, t) = ts_type_to_rs(st, ctxt, types[0], lkm);
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
                            let TypeConvertContext { path } = ctxt.expect("provide ctxt");
                            let tn = name_types::string_literal_union(st, variants, &path);
                            break 't RustType::Custom(tn);
                            //TODO: comment strs  // {:?}", strs));
                        }

                        // other types
                        dbg!(types);

                        RustType::UnknownUnion
                    }
                    TsUnionOrIntersectionType::TsIntersectionType(tints) => {
                        if tints.types.len() == 2 {
                            let mut iter = tints.types.iter();
                            // if types consist of type literal and type ref, create new struct with serde flatten attribute
                            let tref = iter.next().unwrap().as_ts_type_ref();
                            let tlit = iter.next().unwrap().as_ts_type_lit();
                            if let (Some(tref), Some(tlit)) = (tref, tlit) {
                                let name = tref.type_name.as_ident().unwrap().sym.as_ref();
                                let mut str =
                                    name_types::type_literal(st, tlit, ctxt.unwrap(), lkm);

                                if str.member.iter().all(|m| m.ty.is_unknown()) {
                                    let struct_name = str.name.to_owned();
                                    let a = RustAlias {
                                        name: struct_name.to_owned(),
                                        is_borrowed: false,
                                        ty: RustType::Custom(TypeName::new(name.to_owned())),
                                    };
                                    st.segments.push(RustSegment::Alias(a));
                                    break 't RustType::Custom(TypeName::new(struct_name));
                                } else {
                                    // add flatten attributed field to struct
                                    let mut field_name = name.to_owned();
                                    case::CaseConvention::Pascal
                                        .into_rename_rule()
                                        .convert_to_snake(&mut field_name);
                                    str.member.push(RustStructMember {
                                        attr: RustFieldAttrs::from_attr(RustFieldAttr::Serde(
                                            SerdeFieldAttr::Flatten,
                                        )),
                                        name: field_name,
                                        ty: RustMemberType {
                                            is_optional: false,
                                            ty: RustType::Custom(TypeName::new(name.to_owned())),
                                        },
                                        comment: None,
                                    });
                                    let struct_name = str.name.to_owned();
                                    st.segments.push(RustSegment::Struct(str));
                                    break 't RustType::Custom(TypeName::new(struct_name));
                                }
                            }
                        }
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
                let (_n, etype) = ts_type_to_rs(st, ctxt, &tarray.elem_type, lkm);
                //format!("Vec<{etype}>")
                RustType::Array(Box::new(etype))
            }
            swc_ecma_ast::TsType::TsTypeLit(tlit) => {
                let s = name_types::type_literal(st, tlit, ctxt.unwrap(), lkm);
                let name = s.name.clone();
                st.segments.push(RustSegment::Struct(s));

                RustType::Custom(TypeName::new(name))
            }
            _ => {
                //dbg!(typ);
                //todo!();
                RustType::Unknown
            }
        }
    };

    (nullable, typ)
}

pub fn into_pascal(path: &Path) -> String {
    path.iter()
        .map(|p| {
            let mut p = p.to_string();
            case::detect_case(&p)
                .into_rename_rule()
                .convert_to_pascal(&mut p);
            p
        })
        .collect::<Vec<_>>()
        .join("")
}
