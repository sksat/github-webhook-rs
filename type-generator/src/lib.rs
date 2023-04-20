pub mod case;
mod dag;
mod frontend;
pub mod ir;
mod to_tokens;
mod transformer;

use frontend::FrontendState;
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

use ir::{type_deps, LiteralKeyMap, RustAlias, RustSegment, RustType, TypeName};

pub fn dts2rs(dts_file: &PathBuf) -> proc_macro2::TokenStream {
    let module = extract_module(dts_file);

    let mut segments = Vec::new();

    let mut st = FrontendState {
        segments: &mut segments,
        name_types: Default::default(),
    };

    // candidate for discriminated union using literal
    // type name -> prop name -> literal value
    let mut lkm: LiteralKeyMap = HashMap::new();

    for b in &module.body {
        let b = b.as_module_decl().unwrap();
        let b = b.as_export_decl().expect("module have only exports");
        let decl = &b.decl;

        //dbg!(&decl);
        match decl {
            swc_ecma_ast::Decl::TsInterface(interface) => {
                //let name = interface.id.sym.as_ref();
                //match name {
                //    "CheckRunCreatedEvent" | "GollumEvent" => continue,
                //    _ => {}
                //}

                frontend::interface2struct(&mut st, interface, &mut lkm);
            }
            swc_ecma_ast::Decl::TsTypeAlias(talias) => {
                let ident = talias.id.sym.as_ref();

                // lazy skip
                match ident {
                    "WebhookEvents"
                    | "PullRequestReviewRequestRemovedEvent"
                    | "PullRequestReviewRequestedEvent" => {
                        continue; //return Err(anyhow!("lazy skip"));
                    }
                    _ => {}
                }

                let typ = &talias.type_ann;
                match typ.as_ref() {
                    swc_ecma_ast::TsType::TsTypeRef(tref) => {
                        let rhs = tref.type_name.as_ident().unwrap().sym.as_ref();
                        let rhs = rhs.to_owned();
                        let a = RustSegment::Alias(RustAlias {
                            name: ident.to_owned(),
                            is_borrowed: false,
                            ty: RustType::Custom(TypeName {
                                name: rhs,
                                is_borrowed: false,
                            }),
                        });
                        st.segments.push(a);
                    }
                    swc_ecma_ast::TsType::TsUnionOrIntersectionType(tuoi) => {
                        let tunion = tuoi.as_ts_union_type().unwrap();

                        let renum = frontend::tunion2enum(ident, tunion);
                        let e = RustSegment::Enum(renum);
                        st.segments.push(e);
                    }
                    swc_ecma_ast::TsType::TsKeywordType(..)
                    | swc_ecma_ast::TsType::TsArrayType(..) => {
                        // export type Hoge = number;
                        let typ = frontend::ts_type_to_rs(&mut st, None, typ).1;
                        let a = RustSegment::Alias(RustAlias {
                            name: ident.to_owned(),
                            is_borrowed: false,
                            ty: typ,
                        });
                        st.segments.push(a);
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
        //println!("{}", b.is_export_decl());
    }
    // drop(st);

    for segment in &mut segments {
        transformer::adapt_internal_tag(segment, &lkm);
        transformer::adapt_rename_all(segment);
    }
    let type_deps = type_deps(&segments);
    transformer::adapt_borrow(&mut segments, &type_deps);

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
