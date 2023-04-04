use std::path::{Path, PathBuf};

use swc_common::{
    self,
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap,
};

use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax};

use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};

macro_rules! id {
    ($($tt:tt)*) => {
        proc_macro2::Ident::new($($tt)*, proc_macro2::Span::call_site())
    };
}

pub enum RustType {
    String,
    Number,
    Boolean,
    Custom(String),
    Array(Box<RustType>),
    Empty, // ()
    Unknown,
    UnknownLiteral,
    UnknownIntersection,
    UnknownUnion,
}

impl RustType {
    pub fn is_unknown(&self) -> bool {
        match &self {
            RustType::Unknown
            | RustType::UnknownLiteral
            | RustType::UnknownIntersection
            | RustType::UnknownUnion => true,
            RustType::Array(t) => t.is_unknown(),
            _ => false,
        }
    }
}

pub struct RustStruct {
    pub name: String,
    pub member: Vec<RustStructMember>,
}

pub struct RustEnum {
    pub name: String,
    pub member: Vec<RustEnumMember>,
}

pub struct RustStructMember {
    pub name: String,
    pub ty: RustMemberType,
    pub comment: Option<String>,
}

pub struct RustMemberType {
    pub ty: RustType,
    pub is_optional: bool,
}

impl RustMemberType {
    pub fn is_unknown(&self) -> bool {
        self.ty.is_unknown()
    }
}

pub enum RustEnumMember {
    TypeAlias(String),
    EnumVariant(String),
}

impl ToTokens for RustType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let s = match self {
            RustType::String => "String",
            RustType::Number => "usize",
            RustType::Boolean => "bool",
            RustType::Custom(s) => s.as_str(),
            RustType::Array(t) => {
                tokens.extend(
                    quote! {
                        Vec<#t>
                    }
                    .into_iter(),
                );
                return;
            }
            RustType::Empty => {
                tokens.append(TokenTree::Group(proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis,
                    Default::default(),
                )));
                return;
            }
            RustType::Unknown => "Unknown",
            RustType::UnknownLiteral => "UnknownLiteral",
            RustType::UnknownIntersection => "UnknownIntersection",
            RustType::UnknownUnion => "UnknownUnion",
        };
        tokens.append(TokenTree::Ident(id!(s)));
    }
}

impl ToTokens for RustMemberType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner_ty = &self.ty;
        tokens.extend(
            if self.is_optional {
                quote! {
                    Option<#inner_ty>
                }
            } else {
                quote! {
                    #inner_ty
                }
            }
            .into_iter(),
        )
    }
}

impl ToTokens for RustStructMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, ty, comment } = self;
        let name = id!(name);

        tokens.extend(
            if self.ty.is_unknown() {
                quote! {
                    /* unknown type */
                }
            } else if let Some(comment) = comment {
                quote! {
                    #[doc=#comment]
                    pub #name: #ty,
                }
            } else {
                quote! {
                    pub #name: #ty,
                }
            }
            .into_iter(),
        );
    }
}

impl ToTokens for RustStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, member } = self;
        let name = id!(name);

        tokens.extend(
            quote! {
                pub struct #name {
                    #(#member)*
                }
            }
            .into_iter(),
        );
    }
}

impl ToTokens for RustEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, member } = self;
        let name = id!(name);

        tokens.extend(
            quote! {
                pub enum #name {
                    #(#member)*
                }
            }
            .into_iter(),
        );
    }
}

impl ToTokens for RustEnumMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            match self {
                RustEnumMember::EnumVariant(v) => {
                    let v = id!(v);
                    quote!(#v,)
                }
                RustEnumMember::TypeAlias(a) => {
                    let a = id!(a);
                    quote!(#a(#a),)
                }
            }
            .into_iter(),
        )
    }
}

fn interface2struct(interface: &swc_ecma_ast::TsInterfaceDecl) -> RustStruct {
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

        let pkey: &str = match &*prop.key {
            swc_ecma_ast::Expr::Ident(pkey) => &pkey.sym,
            swc_ecma_ast::Expr::Lit(pkey) => match pkey {
                swc_ecma_ast::Lit::Str(_pkey) => {
                    // TODO: use &pkey.value.as_ref()
                    continue;
                }
                _ => unreachable!(),
            },
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

        // avoid conflict to Rust reserved word
        let pkey = match pkey {
            "type" => "type_",
            "ref" => "ref_",
            _ => pkey,
        };

        let ptype = &prop.type_ann.as_ref().unwrap().type_ann;
        let (is_optional, ty) = ts_type_to_rs(ptype);

        rmember.push(RustStructMember {
            ty: RustMemberType { ty, is_optional },
            name: pkey.to_string(),
            comment: None,
        });
    }

    RustStruct {
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
                member.push(RustEnumMember::TypeAlias(sym));
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
                member.push(RustEnumMember::EnumVariant(s));
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
        name: name.to_string(),
        member,
    }
}

pub fn dts2rs(dts_file: &PathBuf) -> TokenStream {
    let module = extract_module(dts_file);

    let mut token_streams = Vec::new();

    // processed later
    // let mut leftovers = Vec::new();

    for b in module.body {
        let b = b.as_module_decl().unwrap();
        let b = b.as_export_decl().expect("module have only exports");
        let decl = &b.decl;

        //dbg!(&decl);
        let token_stream = match decl {
            swc_ecma_ast::Decl::TsInterface(interface) => {
                //let name = interface.id.sym.as_ref();
                //match name {
                //    "CheckRunCreatedEvent" | "GollumEvent" => continue,
                //    _ => {}
                //}

                let rstruct = interface2struct(interface);
                quote! {
                    // ts interface
                    #rstruct
                }
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
                        let rhs = &tref.type_name.as_ident().unwrap().sym.to_string();
                        let rhs = id!(rhs);
                        quote! {
                            // ts type alias of another type
                            pub type #ident = #rhs;
                        }
                    }
                    swc_ecma_ast::TsType::TsUnionOrIntersectionType(tuoi) => {
                        let tunion = tuoi.as_ts_union_type().unwrap();

                        let renum = tunion2enum(a, tunion).into_token_stream();
                        quote! {
                            // ts type alias of union
                            #renum
                        }
                    }
                    swc_ecma_ast::TsType::TsKeywordType(..)
                    | swc_ecma_ast::TsType::TsArrayType(..) => {
                        // export type Hoge = number;
                        let typ = ts_type_to_rs(typ).1;
                        quote! {
                            pub type #ident = #typ;
                        }
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
        token_streams.push(token_stream);
        //println!("{}", b.is_export_decl());
    }

    token_streams.into_iter().collect()
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
                TsUnionOrIntersectionType::TsIntersectionType(tints) => {
                    // dbg!(tints);
                    //todo!();
                    RustType::UnknownIntersection
                }
            }
        }
        swc_ecma_ast::TsType::TsLitType(_tslit) => RustType::UnknownLiteral,
        swc_ecma_ast::TsType::TsTypeRef(tref) => {
            RustType::Custom(tref.type_name.as_ident().unwrap().sym.as_ref().to_string())
        }
        swc_ecma_ast::TsType::TsArrayType(tarray) => {
            let (_n, etype) = ts_type_to_rs(&tarray.elem_type);
            //format!("Vec<{etype}>")
            RustType::Array(Box::new(etype))
        }
        swc_ecma_ast::TsType::TsTypeLit(tlit) => {
            for m in &tlit.members {
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
