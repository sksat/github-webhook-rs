mod dag;
pub mod ir;
mod to_tokens;
mod transformer;

use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use swc_common::{
    self,
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap,
};

use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax};

use ir::*;

macro_rules! id {
    ($($tt:tt)*) => {
        proc_macro2::Ident::new($($tt)*, proc_macro2::Span::call_site())
    };
}

fn interface2struct(
    interface: &swc_ecma_ast::TsInterfaceDecl,
    lkm: &mut LiteralKeyMap,
) -> RustStruct {
    let name = interface.id.sym.to_string();
    let ibody = &interface.body.body;

    let mut rmember = Vec::new();

    for member in ibody {
        let prop = member.as_ts_property_signature().unwrap();
        if prop.optional {
            // TODO: optional or skip
            continue;
        }
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
        let (is_optional, ty) = ts_type_to_rs(ptype);

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

    RustStruct {
        attr: RustContainerAttrs::Default,
        name,
        member: rmember,
    }
}

fn tunion2enum(name: &str, tunion: &swc_ecma_ast::TsUnionType) -> RustEnum {
    let mut member = Vec::new();
    for t in &tunion.types {
        match &**t {
            swc_ecma_ast::TsType::TsTypeRef(tref) => {
                // export type Hoge = Fuga | Fuge;
                let i = &tref.type_name.as_ident().unwrap();
                let sym = i.sym.to_string();
                member.push(RustEnumMember::Unary(sym));
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
                member.push(RustEnumMember::Nullary(s));
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
        member,
    }
}

pub fn dts2rs(dts_file: &PathBuf) -> proc_macro2::TokenStream {
    let module = extract_module(dts_file);

    let mut segments = Vec::new();

    // candidate for discriminated union using literal
    // type name -> prop name -> literal value
    let mut lkm: LiteralKeyMap = HashMap::new();

    for b in module.body {
        let b = b.as_module_decl().unwrap();
        let b = b.as_export_decl().expect("module have only exports");
        let decl = &b.decl;

        //dbg!(&decl);
        let segment = match decl {
            swc_ecma_ast::Decl::TsInterface(interface) => {
                //let name = interface.id.sym.as_ref();
                //match name {
                //    "CheckRunCreatedEvent" | "GollumEvent" => continue,
                //    _ => {}
                //}

                let rstruct = interface2struct(interface, &mut lkm);
                RustSegment::Struct(rstruct)
            }
            swc_ecma_ast::Decl::TsTypeAlias(talias) => {
                let a = talias.id.sym.as_ref();

                // lazy skip
                match a {
                    "WebhookEvents"
                    | "PullRequestReviewRequestRemovedEvent"
                    | "PullRequestReviewRequestedEvent" => {
                        continue; //return Err(anyhow!("lazy skip"));
                    }
                    _ => {}
                }
                let ident = id!(a);

                let typ = &talias.type_ann;
                match typ.as_ref() {
                    swc_ecma_ast::TsType::TsTypeRef(tref) => {
                        let rhs = tref.type_name.as_ident().unwrap().sym.as_ref();
                        let rhs = rhs.to_owned();
                        RustSegment::Alias(ident, RustType::Custom(rhs))
                    }
                    swc_ecma_ast::TsType::TsUnionOrIntersectionType(tuoi) => {
                        let tunion = tuoi.as_ts_union_type().unwrap();

                        let renum = tunion2enum(a, tunion);
                        RustSegment::Enum(renum)
                    }
                    swc_ecma_ast::TsType::TsKeywordType(..)
                    | swc_ecma_ast::TsType::TsArrayType(..) => {
                        // export type Hoge = number;
                        let typ = ts_type_to_rs(typ).1;
                        RustSegment::Alias(ident, typ)
                    }
                    swc_ecma_ast::TsType::TsTypeOperator(_toperator) => {
                        // export type WebhookEventName = keyof EventPayloadMap;
                        //dbg!(toperator);
                        continue;
                    }
                    _ => {
                        dbg!(typ);
                        unreachable!()
                    }
                }
            }
            _ => unreachable!(),
        };
        segments.push(segment);
        //println!("{}", b.is_export_decl());
    }

    for segment in &mut segments {
        transformer::adapt_internal_tag(segment, &lkm);
        transformer::adapt_rename_all(segment);
    }

    segments
        .into_iter()
        .flat_map(|rss| rss.into_token_stream())
        .collect()
}

fn extract_module(dts_file: &PathBuf) -> swc_ecma_ast::Module {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    // Real usage
    let fm = cm
        .load_file(Path::new(dts_file))
        .unwrap_or_else(|_| panic!("failed to load {}", &dts_file.display()));

    let lexer = Lexer::new(
        Syntax::Typescript(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let capturing = Capturing::new(lexer);

    let mut parser = Parser::new_from(capturing);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    parser
        .parse_typescript_module()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("Failed to parse module.")
}

fn ts_keyword_type_to_rs(typ: &swc_ecma_ast::TsKeywordType) -> RustType {
    use swc_ecma_ast::TsKeywordTypeKind;
    match typ.kind {
        TsKeywordTypeKind::TsStringKeyword => RustType::String,
        TsKeywordTypeKind::TsNumberKeyword => RustType::Number,
        TsKeywordTypeKind::TsBooleanKeyword => RustType::Boolean,
        TsKeywordTypeKind::TsNullKeyword => RustType::Empty,
        _ => {
            unimplemented!("{:?}", typ.kind);
        }
    }
}

fn ts_type_to_rs(typ: &swc_ecma_ast::TsType) -> (bool, RustType) {
    use swc_ecma_ast::TsKeywordTypeKind;
    use swc_ecma_ast::TsUnionOrIntersectionType;

    let mut nullable = false;

    let typ = match typ {
        swc_ecma_ast::TsType::TsKeywordType(tk) => {
            let t = ts_keyword_type_to_rs(tk);
            if let RustType::Empty = &t {
                nullable = true;
            }
            t
        }
        swc_ecma_ast::TsType::TsUnionOrIntersectionType(tsuoi) => {
            match tsuoi {
                TsUnionOrIntersectionType::TsUnionType(tunion) => {
                    // nullable check
                    let mut types = tunion.types.clone();
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
                        let (n, t) = ts_type_to_rs(&types[0]);
                        return (n || nullable, t);
                    }

                    // strings check: "Bot" | "User" | "Organization"
                    if types.iter().all(|t| {
                        if let Some(t) = t.as_ts_lit_type() {
                            if t.lit.is_str() {
                                return true;
                            }
                        }
                        false
                    }) {
                        let _strs = types
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
                            .collect::<Vec<&str>>();
                        return (nullable, RustType::String);
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
            RustType::Custom(id.to_owned())
        }
        swc_ecma_ast::TsType::TsArrayType(tarray) => {
            let (_n, etype) = ts_type_to_rs(&tarray.elem_type);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module() {
        let module = extract_module(&PathBuf::from("test.ts"));

        let ice = module.body[1]
            .as_module_decl()
            .unwrap()
            .as_export_decl()
            .unwrap()
            .decl
            .as_ts_type_alias()
            .unwrap()
            .type_ann
            .as_ts_union_or_intersection_type()
            .unwrap()
            .as_ts_union_type()
            .unwrap()
            .types[0]
            .as_ts_type_ref()
            .unwrap()
            .type_name
            .as_ident()
            .unwrap()
            .as_ref();
        assert_eq!(ice, "IssueCommentCreatedEvent");
    }
}
