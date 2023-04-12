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
    Custom(TokenStream),
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
    pub attr: RustContainerAttrs,
    pub name: String,
    pub member: Vec<RustStructMember>,
}

pub enum RustContainerAttrs {
    Default,
    With(Vec<RustStructAttr>),
}

impl RustContainerAttrs {
    fn add_attr(&mut self, a: RustStructAttr) {
        match self {
            RustContainerAttrs::Default => *self = Self::With(vec![a]),
            RustContainerAttrs::With(v) => v.push(a),
        }
    }
    fn is_tagged_enum(&self) -> bool {
        match self {
            RustContainerAttrs::Default => false,
            RustContainerAttrs::With(attrs) => attrs
                .iter()
                .filter_map(|attr| attr.as_serde())
                .any(SerdeContainerAttr::is_tag),
        }
    }
}

pub enum RustStructAttr {
    Serde(SerdeContainerAttr),
}

impl RustStructAttr {
    pub fn as_serde(&self) -> Option<&SerdeContainerAttr> {
        let Self::Serde(v) = self;
        Some(v)
    }
}

impl ToTokens for RustStructAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            match self {
                RustStructAttr::Serde(s) => quote! {
                    #[serde(#s)]
                },
            }
            .into_iter(),
        )
    }
}

pub enum SerdeContainerAttr {
    RenameAll(RenameRule),
    Tag(String),
}

impl SerdeContainerAttr {
    /// Returns `true` if the serde container attr is [`Tag`].
    ///
    /// [`Tag`]: SerdeContainerAttr::Tag
    #[must_use]
    pub fn is_tag(&self) -> bool {
        matches!(self, Self::Tag(..))
    }
}

impl ToTokens for SerdeContainerAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            match self {
                SerdeContainerAttr::RenameAll(r) => {
                    let r = r.to_string();
                    quote! {
                        rename_all = #r
                    }
                }
                SerdeContainerAttr::Tag(name) => quote! {
                    tag = #name
                },
            }
            .into_iter(),
        )
    }
}

pub enum SerdeFieldAttr {
    Rename(String),
}

pub enum SerdeVariantAttr {
    Rename(String),
    Borrow {
        /// lifetime parameter without `'`
        lifetime: String,
    },
}

pub enum RenameRule {
    PascalCase,
    SnakeCase,
    ScreamingSnakeCase,
}

impl RenameRule {
    /// Returns `true` if the rename rule is [`PascalCase`].
    ///
    /// [`PascalCase`]: RenameRule::PascalCase
    #[must_use]
    pub fn is_pascal_case(&self) -> bool {
        matches!(self, Self::PascalCase)
    }
    fn convert_to_pascal(&self, s: &mut String) {
        match self {
            RenameRule::PascalCase => (),
            RenameRule::SnakeCase | RenameRule::ScreamingSnakeCase => {
                *s = s
                    .split('_')
                    .map(|term| {
                        let mut term = term.to_ascii_lowercase();
                        if let Some(c) = term.chars().next() {
                            let capital_ch = c.to_ascii_uppercase();
                            term.replace_range(..1, &capital_ch.to_string());
                        }
                        term
                    })
                    .collect::<Vec<_>>()
                    .concat();
            }
        }
    }
}

impl ToString for RenameRule {
    fn to_string(&self) -> String {
        match self {
            RenameRule::PascalCase => "PascalCase",
            RenameRule::SnakeCase => "snake_case",
            RenameRule::ScreamingSnakeCase => "SCREAMING_SNAKE_CASE",
        }
        .to_string()
    }
}

pub struct RustEnum {
    pub attr: RustContainerAttrs,
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
    Nullary(String),
    /// has the same ident. this is unary
    Unary(String),
    UnaryNamed {
        variant_name: String,
        type_name: String,
    },
}

impl RustEnumMember {
    fn name_unary(&mut self, variant_name: String) {
        match self {
            RustEnumMember::Unary(u) => {
                *self = Self::UnaryNamed {
                    variant_name,
                    type_name: u.clone(),
                }
            }
            _ => unreachable!("do not call with this"),
        }
    }

    /// Returns `true` if the rust enum member is [`Nullary`].
    ///
    /// [`Nullary`]: RustEnumMember::Nullary
    #[must_use]
    pub fn is_nullary(&self) -> bool {
        matches!(self, Self::Nullary(..))
    }

    pub fn as_unary(&self) -> Option<&String> {
        if let Self::Unary(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl ToTokens for RustType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let s = match self {
            RustType::String => "String",
            RustType::Number => "usize",
            RustType::Boolean => "bool",
            RustType::Custom(s) => {
                tokens.extend(s.clone().into_iter());
                return;
            }
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
        let Self { name, member, attr } = self;
        let name = id!(name);
        tokens.extend(
            match attr {
                RustContainerAttrs::Default => quote! {
                    #[derive(Debug, Deserialize)]
                },
                RustContainerAttrs::With(w) => quote! {
                    #[derive(Debug, Deserialize)]
                    #(#w)*
                },
            }
            .into_iter(),
        );

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
        let Self { name, member, attr } = self;
        let name = id!(name);
        tokens.extend(
            if attr.is_tagged_enum() || member.iter().all(|m| m.is_nullary()) {
                quote! {
                    #[derive(Debug, Deserialize)]
                }
            } else {
                quote! {
                    #[derive(Debug)]
                }
            }
            .into_iter(),
        );
        match attr {
            RustContainerAttrs::Default => (),
            RustContainerAttrs::With(w) => {
                tokens.extend(
                    quote! {
                        #(#w)*
                    }
                    .into_iter(),
                );
            }
        }

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
                RustEnumMember::Nullary(v) => {
                    let v = id!(v);
                    quote!(#v,)
                }
                RustEnumMember::Unary(a) => {
                    let a = id!(a);
                    quote!(#a(#a),)
                }
                RustEnumMember::UnaryNamed {
                    variant_name,
                    type_name,
                } => {
                    let variant_name = id!(variant_name);
                    let type_name = id!(type_name);
                    quote!(#variant_name(#type_name),)
                }
            }
            .into_iter(),
        )
    }
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

enum RustSegment {
    Struct(RustStruct),
    Enum(RustEnum),
    Alias(proc_macro2::Ident, RustType),
}

impl RustSegment {
    fn into_token_stream(self) -> TokenStream {
        match self {
            RustSegment::Struct(s) => s.into_token_stream(),
            RustSegment::Enum(e) => e.into_token_stream(),
            RustSegment::Alias(ident, typ) => quote! {
                pub type #ident = #typ;
            },
        }
    }
}

type LiteralKeyMap = HashMap<String, HashMap<String, String>>;

pub fn dts2rs(dts_file: &PathBuf) -> TokenStream {
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
                        let rhs = &tref.type_name.as_ident().unwrap().sym;
                        let rhs = id!(rhs);
                        RustSegment::Alias(ident, RustType::Custom(quote!(#rhs)))
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
        adapt_internal_tag(segment, &lkm);
        adapt_rename_all(segment);
    }

    segments
        .into_iter()
        .flat_map(|rss| rss.into_token_stream())
        .collect()
}

fn adapt_internal_tag(segment: &mut RustSegment, lkm: &LiteralKeyMap) -> Option<()> {
    if let RustSegment::Enum(re) = segment {
        let mut props: HashMap<String, String> = Default::default();
        for memb in &re.member {
            let inter = memb.as_unary()?;
            let map = lkm.get(inter)?;
            if props.is_empty() {
                props = map.clone();
                continue;
            }
            props.retain(|k, _| map.contains_key(k));
            if props.is_empty() {
                return None;
            }
        }
        // assert !props.is_empty()
        assert_eq!(props.len(), 1);
        let tag_name = props.keys().next().unwrap().to_owned();
        for memb in &mut re.member {
            let inter = memb.as_unary().unwrap();
            let variant_name = lkm.get(inter).unwrap().get(&tag_name).unwrap().to_owned();
            memb.name_unary(variant_name);
        }
        re.attr
            .add_attr(RustStructAttr::Serde(SerdeContainerAttr::Tag(tag_name)));
    }
    Some(())
}

fn adapt_rename_all(segment: &mut RustSegment) -> Option<()> {
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum CaseConvention {
        Lower,
        Upper,
        Snake,
        Pascal,
        ScreamingSnake,
    }
    use CaseConvention::*;
    /// expect input follows one of above case
    fn detect_case(s: &str) -> CaseConvention {
        if s.starts_with(char::is_uppercase) {
            if s.contains('_') {
                ScreamingSnake
            } else if s.chars().all(char::is_uppercase) {
                Upper
            } else {
                Pascal
            }
        } else if s.contains('_') {
            Snake
        } else {
            Lower
        }
    }
    impl CaseConvention {
        fn cast(&mut self, other: Self) -> Option<()> {
            if self == &other {
                return Some(());
            }
            match (&self, &other) {
                (Lower, Snake) | (Upper, ScreamingSnake) => {
                    *self = other;
                }
                (Snake, Lower) | (ScreamingSnake, Upper) => {
                    // do nothing
                }
                _ => None?,
            }
            Some(())
        }
        fn into_rename_rule(self) -> RenameRule {
            match self {
                Lower | Snake => RenameRule::SnakeCase,
                Pascal => RenameRule::PascalCase,
                Upper | ScreamingSnake => RenameRule::ScreamingSnakeCase,
            }
        }
    }
    if let RustSegment::Enum(re) = segment {
        let mut conv: Option<CaseConvention> = None;
        for memb in &re.member {
            let s = match memb {
                RustEnumMember::Nullary(v) => v,
                RustEnumMember::Unary(v) => v,
                RustEnumMember::UnaryNamed { variant_name, .. } => variant_name,
            };
            match conv.as_mut() {
                Some(conv) => {
                    let new = detect_case(s);
                    conv.cast(new)?
                }
                None => {
                    conv = Some(detect_case(s));
                }
            };
        }
        let rr = conv?.into_rename_rule();
        if rr.is_pascal_case() {
            return None;
        }
        for memb in &mut re.member {
            match memb {
                RustEnumMember::Unary(v) => {
                    let type_name = v.to_owned();
                    rr.convert_to_pascal(v);
                    *memb = RustEnumMember::UnaryNamed {
                        variant_name: v.to_owned(),
                        type_name,
                    };
                }
                RustEnumMember::Nullary(variant_name)
                | RustEnumMember::UnaryNamed { variant_name, .. } => {
                    rr.convert_to_pascal(variant_name);
                }
            };
        }
        re.attr
            .add_attr(RustStructAttr::Serde(SerdeContainerAttr::RenameAll(rr)));
    }
    Some(())
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
            let id = id!(tref.type_name.as_ident().unwrap().sym.as_ref());
            RustType::Custom(quote!(#id))
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
