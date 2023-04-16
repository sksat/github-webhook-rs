use std::collections::HashMap;

use crate::dag::CoDirectedAcyclicGraph;

pub enum RustSegment {
    Struct(RustStruct),
    Enum(RustEnum),
    Alias(RustAlias),
}

impl RustSegment {
    pub fn name(&self) -> &str {
        match self {
            RustSegment::Struct(s) => &s.name,
            RustSegment::Enum(e) => &e.name,
            RustSegment::Alias(a) => &a.name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeName {
    pub name: String,
    pub is_borrowed: bool,
}

impl TypeName {
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_borrowed: false,
        }
    }
}

#[derive(Debug)]
pub enum RustType {
    String {
        is_borrowed: bool,
    },
    Number,
    Boolean,
    Custom(TypeName),
    Array(Box<RustType>),
    /// `()`
    Unit,
    /// union that consists of string literal
    StringLiteralUnion(Vec<String>),
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

    pub fn as_custom(&self) -> Option<&TypeName> {
        if let Self::Custom(t) = self {
            Some(t)
        } else {
            None
        }
    }

    pub fn get_using(&self) -> Option<&TypeName> {
        if let Self::Array(t) = self {
            t.get_using()
        } else {
            self.as_custom()
        }
    }

    /// Returns `true` if the rust type is [`String`].
    ///
    /// [`String`]: RustType::String
    #[must_use]
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String { .. })
    }
}

pub struct RustStruct {
    pub attr: RustContainerAttrs,
    pub name: String,
    pub is_borrowed: bool,
    pub member: Vec<RustStructMember>,
}

pub type RustContainerAttrs = Attrs<RustStructAttr>;

impl RustContainerAttrs {
    pub fn is_tagged_enum(&self) -> bool {
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

pub enum SerdeFieldAttr {
    Rename(String),
    Borrow,
}

pub enum SerdeVariantAttr {
    Rename(String),
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
    pub fn convert_to_pascal(&self, s: &mut String) {
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
    pub is_borrowed: bool,
    pub member: Vec<RustEnumMember>,
}

impl RustEnum {
    pub fn from_members(name: String, members: impl Iterator<Item = RustEnumMember>) -> Self {
        Self {
            attr: RustContainerAttrs::Default,
            name,
            is_borrowed: false,
            member: members.collect(),
        }
    }
}

pub struct RustStructMember {
    pub attr: RustFieldAttrs,
    pub name: String,
    pub ty: RustMemberType,
    pub comment: Option<String>,
}

pub type RustFieldAttrs = Attrs<RustFieldAttr>;

#[derive(Default)]
pub enum Attrs<Field> {
    #[default]
    Default,
    With(Vec<Field>),
}

impl<T> Attrs<T> {
    pub fn add_attr(&mut self, a: T) {
        match self {
            Self::Default => *self = Self::With(vec![a]),
            Self::With(v) => v.push(a),
        }
    }
}

pub enum RustFieldAttr {
    Serde(SerdeFieldAttr),
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

pub struct RustEnumMember {
    pub attr: RustVariantAttrs,
    pub kind: RustEnumMemberKind,
}

impl From<RustEnumMemberKind> for RustEnumMember {
    fn from(value: RustEnumMemberKind) -> Self {
        Self {
            attr: RustVariantAttrs::Default,
            kind: value,
        }
    }
}

pub enum RustEnumMemberKind {
    Nullary(TypeName),
    /// has the same ident. this is unary
    Unary(TypeName),
    UnaryNamed {
        variant_name: String,
        type_name: TypeName,
    },
}

impl RustEnumMemberKind {
    pub fn name_unary(&mut self, variant_name: String) {
        match self {
            RustEnumMemberKind::Unary(u) => {
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

    pub fn as_unary(&self) -> Option<&TypeName> {
        if let Self::Unary(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn type_name(&self) -> &TypeName {
        match self {
            RustEnumMemberKind::Nullary(t) => t,
            RustEnumMemberKind::Unary(t) => t,
            RustEnumMemberKind::UnaryNamed { type_name, .. } => type_name,
        }
    }

    pub fn type_name_mut(&mut self) -> &mut TypeName {
        match self {
            RustEnumMemberKind::Nullary(t) => t,
            RustEnumMemberKind::Unary(t) => t,
            RustEnumMemberKind::UnaryNamed { type_name, .. } => type_name,
        }
    }
}

pub type RustVariantAttrs = Attrs<RustVariantAttr>;

pub enum RustVariantAttr {
    Serde(SerdeVariantAttr),
}

pub struct RustAlias {
    pub name: String,
    pub is_borrowed: bool,
    pub ty: RustType,
}

pub type LiteralKeyMap = HashMap<String, HashMap<String, String>>;

pub fn type_deps(segments: &[RustSegment]) -> CoDirectedAcyclicGraph<usize> {
    let index_map: HashMap<_, _> = segments
        .iter()
        .enumerate()
        .map(|(i, s)| (s.name(), i))
        .collect();
    let mut type_deps = CoDirectedAcyclicGraph::new();
    for (i, segment) in segments.iter().enumerate() {
        let children: Vec<_> = match segment {
            RustSegment::Struct(s) => s
                .member
                .iter()
                .flat_map(|m| m.ty.ty.get_using())
                .map(|t| t.name.as_str())
                .collect(),
            RustSegment::Enum(e) => e
                .member
                .iter()
                .map(|m| m.kind.type_name().name.as_str())
                .collect(),
            RustSegment::Alias(a) => {
                a.ty.get_using()
                    .map(|t| t.name.as_str())
                    .into_iter()
                    .collect()
            }
        };
        for child in children {
            if let Some(to) = index_map.get(child) {
                type_deps.add_edge(i, *to);
            }
        }
    }
    type_deps
}
