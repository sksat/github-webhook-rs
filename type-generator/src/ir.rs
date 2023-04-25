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

#[derive(Debug, Clone, Default)]
pub enum RustType {
    String {
        is_borrowed: bool,
    },
    Number,
    Boolean,
    Custom(TypeName),
    Array(Box<RustType>),
    Map(Box<Self>, Box<Self>),
    /// `()`
    #[default]
    Unit,
    Unknown,
    UnknownLiteral,
    UnknownIntersection,
}

impl RustType {
    pub fn to_ident(&self) -> &str {
        match self {
            RustType::String { .. } => "String",
            RustType::Number => "Number",
            RustType::Boolean => "Boolean",
            RustType::Custom(c) => &c.name,
            RustType::Array(t) => t.to_ident(),
            RustType::Unit => "Unit",
            RustType::Unknown => "Unknown",
            RustType::UnknownLiteral => "UnknownLiteral",
            RustType::UnknownIntersection => "UnknownIntersection",
            RustType::Map(..) => "Map",
        }
    }

    pub fn is_unknown(&self) -> bool {
        match &self {
            RustType::UnknownLiteral | RustType::UnknownIntersection => true,
            RustType::Array(t) => t.is_unknown(),
            RustType::Map(t1, t2) => t1.is_unknown() || t2.is_unknown(),
            RustType::Unknown
            | RustType::String { .. }
            | RustType::Number
            | RustType::Boolean
            | RustType::Custom(_)
            | RustType::Unit => false,
        }
    }

    pub fn is_borrowed(&self) -> bool {
        match self {
            RustType::String { is_borrowed } => *is_borrowed,
            RustType::Custom(t) => t.is_borrowed,
            RustType::Array(t) => t.is_borrowed(),
            RustType::Map(t1, t2) => t1.is_borrowed() || t2.is_borrowed(),
            RustType::Number
            | RustType::Boolean
            | RustType::Unit
            | RustType::Unknown
            | RustType::UnknownLiteral
            | RustType::UnknownIntersection => false,
        }
    }

    pub fn as_custom(&self) -> Option<&TypeName> {
        if let Self::Custom(t) = self {
            Some(t)
        } else {
            None
        }
    }

    pub fn as_mut_custom(&mut self) -> Option<&mut TypeName> {
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

impl RustStruct {
    pub fn from_members(name: String, members: impl Iterator<Item = RustStructMember>) -> Self {
        Self {
            attr: RustContainerAttrs::new(),
            name,
            is_borrowed: false,
            member: members.collect(),
        }
    }
}

pub type RustContainerAttrs = Attrs<RustStructAttr>;

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
    Untagged,
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

#[derive(PartialEq)]
pub enum SerdeFieldAttr {
    Rename(String),
    Flatten,
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
    pub fn convert_to_snake(&self, s: &mut String) {
        match self {
            RenameRule::PascalCase => {
                *s = s
                    .chars()
                    .enumerate()
                    .fold(String::new(), |mut snake, (i, c)| {
                        if i > 0 && c.is_uppercase() {
                            snake.push('_');
                        }
                        snake.push(c.to_ascii_lowercase());
                        snake
                    });
            }
            _ => unimplemented!(),
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
            attr: RustContainerAttrs::new(),
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
pub struct Attrs<Field>(Vec<Field>);

impl<T> Attrs<T> {
    pub fn add_attr(&mut self, a: T) {
        self.0.push(a)
    }
    pub fn from_attr(a: T) -> Self {
        let mut s = Self::new();
        s.add_attr(a);
        s
    }
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn as_inner(&self) -> &Vec<T> {
        &self.0
    }

    pub fn retain<F: FnMut(&T) -> bool>(&mut self, f: F) {
        self.0.retain(f)
    }
}

#[derive(PartialEq)]
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
            attr: RustVariantAttrs::new(),
            kind: value,
        }
    }
}

pub enum RustEnumMemberKind {
    Nullary(String),
    /// has the same ident. this is unary
    Unary(RustType),
    UnaryNamed {
        variant_name: String,
        type_name: RustType,
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

    pub fn as_unary(&self) -> Option<&RustType> {
        if let Self::Unary(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_type(&self) -> Option<&RustType> {
        match self {
            RustEnumMemberKind::Nullary(..) => None,
            RustEnumMemberKind::Unary(t) => Some(t),
            RustEnumMemberKind::UnaryNamed { type_name, .. } => Some(type_name),
        }
    }

    pub fn as_type_mut(&mut self) -> Option<&mut RustType> {
        match self {
            RustEnumMemberKind::Nullary(..) => None,
            RustEnumMemberKind::Unary(t) => Some(t),
            RustEnumMemberKind::UnaryNamed { type_name, .. } => Some(type_name),
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
                .flat_map(|m| m.kind.as_type())
                .flat_map(|m| m.as_custom())
                .map(|tn| tn.name.as_str())
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
