//! Intermediate representation of types used to deserialize GitHub Webhook payloads.
//!
//! This IR is independent of JSON Schema or Rust naming rules, but assumes
//! the types are intended for use with `serde` and represent structurally valid
//! Rust data models.

use std::borrow::Cow;

use crate::{
    context::Context,
    intern::{Interned, Interner, Str},
};

#[derive(Debug)]
/// The intermediate representation of a module containing schema definitions.
pub struct Module<'cx> {
    /// Definitions in this module.
    pub definitions: Vec<Definition<'cx>>,

    /// Possible variants for the payload type in this module. Recall that
    /// this project generates a type for the entire payload.
    pub variants: Vec<Path<'cx>>,
}

#[derive(Debug)]
/// A definition in the IR, representing a schema-defined type.
pub struct Definition<'cx> {
    /// An optional documentation for this definition.
    pub doc: Doc<'cx>,

    /// The type identifier for this definition.
    pub ty: Ty<'cx>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// An interned type representing a schema-defined type.
pub struct Ty<'cx> {
    pub kind: Interned<TyInterner<'cx>>,
    pub path: Path<'cx>,
}

impl<'cx> std::ops::Deref for Ty<'cx> {
    type Target = Interned<TyInterner<'cx>>;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

pub type TyInterner<'cx> = Interner<'cx, TyKind<'cx>>;

/// An interned absolute path to a schema-defined type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Path<'cx>(pub Interned<PathInterner<'cx>>);

impl<'cx> std::ops::Deref for Path<'cx> {
    type Target = Interned<PathInterner<'cx>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type PathInterner<'cx> = Interner<'cx, DefinitionPath<'cx>>;

/// An interned relative path to a field within a schema-defined type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldPath<'cx>(pub Interned<FieldPathInterner<'cx>>);

impl<'cx> std::ops::Deref for FieldPath<'cx> {
    type Target = Interned<FieldPathInterner<'cx>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type FieldPathInterner<'cx> = Interner<'cx, DefinitionFieldPath<'cx>>;

/// Identifies a schema-defined type by its root name and field path.
///
/// This path describes the origin of a type in terms of its position within a
/// structured schema. It begins at a root definition and descends through a
/// sequence of object fields.
///
/// This is an absolute path; see [`FieldPath`] for relative paths from a type.
///
/// ## Example layout
///
/// Taking an example from JSON Schema frontend:
///
/// ```ignore
///  ┌────────────────────────── definition ─────────────────────────────────────────┐
///  #/definitions/branch_protection_rule$edited/properties/changes/properties/from
///                ▲ base                ▲ optional variant ▲ field            ▲ field
/// ```
pub type DefinitionPath<'cx> = SinglyLinkedListPath<'cx, DefinitionRoot<'cx>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SinglyLinkedListPath<'cx, Root> {
    /// A root path.
    Root(Root),

    /// A path that descends through a field of a parent.
    Field {
        /// The parent path.
        ///
        /// This field refers to `Self` reluctantly because type alias cannot
        /// recurse mutually.
        parent: Interned<Interner<'cx, Self>>,

        /// The field name within the parent.
        field: Str<'cx>,
    },
}

impl<'cx, Root> SinglyLinkedListPath<'cx, Root> {
    pub fn as_root(&self) -> Option<&Root> {
        if let Self::Root(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn root(&self) -> &Root {
        match self {
            Self::Root(v) => v,
            Self::Field { parent, .. } => parent.root(),
        }
    }
}

impl<'cx> Path<'cx> {
    /// Creates a new root path with the given definition root.
    pub fn mk_root(cx: &Context<'cx>, root: DefinitionRoot<'cx>) -> Self {
        cx.intern_path(SinglyLinkedListPath::Root(root))
    }

    /// Creates a new field path from an existing parent path and field name.
    pub fn descend(self, cx: &Context<'cx>, field: Str<'cx>) -> Self {
        cx.intern_path(SinglyLinkedListPath::Field {
            parent: self.0,
            field,
        })
    }
}

pub type DefinitionFieldPath<'cx> = SinglyLinkedListPath<'cx, ()>;

impl<'cx> FieldPath<'cx> {
    /// Creates a new root field path.
    pub fn mk_root(cx: &Context<'cx>) -> FieldPath<'cx> {
        cx.intern_field_path(SinglyLinkedListPath::Root(()))
    }

    /// Creates a new field path from an existing parent path and field name.
    pub fn descend(self, cx: &Context<'cx>, field: Str<'cx>) -> FieldPath<'cx> {
        cx.intern_field_path(SinglyLinkedListPath::Field {
            parent: self.0,
            field,
        })
    }
}

/// A root definition name from the schema, optionally including a variant tag.
///
/// For example, the key `pull_request$opened` is interpreted as:
/// - `base`: `"pull_request"`
/// - `variant`: `Some("opened")`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DefinitionRoot<'cx> {
    /// The main name of the schema definition (e.g., `"pull_request"`).
    pub base: Str<'cx>,

    /// An optional discriminant tag, used in tagged unions (e.g., `"opened"`).
    pub variant: Option<Str<'cx>>,
}

/// A human-readable comment block describing a type or member.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Doc<'cx> {
    pub title: Option<Cow<'cx, str>>,
    pub description: Option<Cow<'cx, str>>,
}

impl<'cx> Doc<'cx> {
    /// Returns `true` if the documentation is empty.
    pub fn is_empty(&self) -> bool {
        self.title.is_none() && self.description.is_none()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
/// A struct-like object with named fields.
pub struct Struct<'cx> {
    pub fields: Vec<Field<'cx>>,
    pub additional: Additional<'cx>,
}

/// A field within a struct.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Field<'cx> {
    /// Original name in the schema.
    pub name: Str<'cx>,

    /// The type of the field.
    pub ty: Ty<'cx>,

    /// Whether this field is required to be present.
    pub required: bool,

    /// Optional documentation.
    pub doc: Doc<'cx>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
/// A fixed string, integer, or boolean enum.
pub struct ConstEnum<'cx> {
    pub variants: Vec<ConstEnumVariant<'cx>>,
}

/// A fixed constant variant in an enum.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ConstEnumVariant<'cx> {
    /// The literal value.
    pub value: Str<'cx>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
/// A tagged union with a discriminant field.
///
/// The tag field is always "action" in GitHub schemas.
pub struct TaggedEnum<'cx> {
    pub variants: Vec<TaggedEnumVariant<'cx>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
/// A variant in a tagged union.
pub struct TaggedEnumVariant<'cx> {
    pub tag_value: Str<'cx>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
/// An untagged union type with a fixed set of variants.
pub struct UntaggedEnum<'cx> {
    pub variants: Vec<Ty<'cx>>,
}

/// A composable type expression usable in Rust and `serde`.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TyKind<'cx> {
    /// A reference to a named type (defined elsewhere).
    Ident(Path<'cx>),

    /// A fixed string, integer, or boolean enum.
    ConstEnum(ConstEnum<'cx>),

    /// A tagged union with a discriminant field.
    TaggedEnum(TaggedEnum<'cx>),

    /// An untagged union type with a fixed set of variants.
    UntaggedEnum(UntaggedEnum<'cx>),

    /// A struct-like object with named fields.
    Struct(Struct<'cx>),

    /// A type that overrides a field in a base type.
    Override(Override<'cx>),

    /// A primitive leaf type.
    Primitive(Primitive),

    /// An optional value.
    Option(Ty<'cx>),

    /// A list of homogeneous items.
    Vec(Ty<'cx>),

    /// A vector of heterogeneous items.
    AnyVec,
}

impl<'cx> Ty<'cx> {
    fn from_kind(cx: &Context<'cx>, kind: TyKind<'cx>, path: Path<'cx>) -> Self {
        Self {
            kind: cx.intern_ty(kind),
            path,
        }
    }

    /// Creates a new type that references a named type.
    pub fn mk_ident(cx: &Context<'cx>, referee: Path<'cx>, path: Path<'cx>) -> Ty<'cx> {
        Self::from_kind(cx, TyKind::Ident(referee), path)
    }

    /// Creates a new type that has a fixed set of variants.
    pub fn mk_const_enum(
        cx: &Context<'cx>,
        variants: Vec<ConstEnumVariant<'cx>>,
        path: Path<'cx>,
    ) -> Ty<'cx> {
        Self::from_kind(cx, TyKind::ConstEnum(ConstEnum { variants }), path)
    }

    /// Creates a new tagged union type with a discriminant field.
    pub fn mk_tagged_enum(
        cx: &Context<'cx>,
        variants: Vec<TaggedEnumVariant<'cx>>,
        path: Path<'cx>,
    ) -> Ty<'cx> {
        Self::from_kind(cx, TyKind::TaggedEnum(TaggedEnum { variants }), path)
    }

    /// Creates a new untagged union type with a fixed set of variants.
    pub fn mk_untagged_enum(cx: &Context<'cx>, variants: Vec<Ty<'cx>>, path: Path<'cx>) -> Ty<'cx> {
        Self::from_kind(cx, TyKind::UntaggedEnum(UntaggedEnum { variants }), path)
    }

    /// Creates a new struct-like type with named fields.
    pub fn mk_struct(
        cx: &Context<'cx>,
        fields: Vec<Field<'cx>>,
        additional: Additional<'cx>,
        path: Path<'cx>,
    ) -> Ty<'cx> {
        Self::from_kind(cx, TyKind::Struct(Struct { fields, additional }), path)
    }

    /// Creates a new override type that modifies a field in a base type.
    pub fn mk_override(cx: &Context<'cx>, override_: Override<'cx>, path: Path<'cx>) -> Ty<'cx> {
        Self::from_kind(cx, TyKind::Override(override_), path)
    }

    /// Creates a new primitive type.
    pub fn mk_primitive(cx: &Context<'cx>, primitive: Primitive, path: Path<'cx>) -> Ty<'cx> {
        Self::from_kind(cx, TyKind::Primitive(primitive), path)
    }

    /// Creates a new string type with an optional format.
    pub fn mk_string(cx: &Context<'cx>, format: Option<StringFormat>, path: Path<'cx>) -> Ty<'cx> {
        Self::from_kind(cx, TyKind::Primitive(Primitive::String { format }), path)
    }

    /// Creates a new boolean type with an optional constant value.
    pub fn mk_boolean(cx: &Context<'cx>, constant: Option<bool>, path: Path<'cx>) -> Ty<'cx> {
        Self::from_kind(cx, TyKind::Primitive(Primitive::Boolean { constant }), path)
    }

    /// Creates a new integer type with an optional constant value.
    pub fn mk_integer(cx: &Context<'cx>, constant: Option<i64>, path: Path<'cx>) -> Ty<'cx> {
        Self::from_kind(cx, TyKind::Primitive(Primitive::Integer { constant }), path)
    }

    /// Creates a new number type.
    pub fn mk_number(cx: &Context<'cx>, path: Path<'cx>) -> Ty<'cx> {
        Self::from_kind(cx, TyKind::Primitive(Primitive::Number), path)
    }

    /// Creates a new null type.
    pub fn mk_null(cx: &Context<'cx>, path: Path<'cx>) -> Ty<'cx> {
        Self::from_kind(cx, TyKind::Primitive(Primitive::Null), path)
    }

    /// Creates a new optional type that wraps another type.
    pub fn mk_option(cx: &Context<'cx>, inner: Ty<'cx>, path: Path<'cx>) -> Ty<'cx> {
        Self::from_kind(cx, TyKind::Option(inner), path)
    }

    /// Creates a new vector type that contains homogeneous items of the given type.
    pub fn mk_vec(cx: &Context<'cx>, inner: Ty<'cx>, path: Path<'cx>) -> Ty<'cx> {
        Self::from_kind(cx, TyKind::Vec(inner), path)
    }

    /// Creates a new vector type that can contain heterogeneous items.
    pub fn mk_any_vec(cx: &Context<'cx>, path: Path<'cx>) -> Ty<'cx> {
        Self::from_kind(cx, TyKind::AnyVec, path)
    }

    pub fn is_null(&self) -> bool {
        matches!(self.kind.0, TyKind::Primitive(Primitive::Null))
    }
}

/// Extra keys allowed in a struct object.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Additional<'cx> {
    /// Disallows arbitrary keys.
    None,

    /// All additional keys must have this type.
    Typed(Ty<'cx>),

    /// Allows arbitrary keys.
    Any,
}

/// A type that overrides a field in a base type.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Override<'cx> {
    /// The base type to which this override applies.
    pub base_ty: Path<'cx>,

    /// A list of override fields that modify the base type.
    pub fields: Vec<OverrideField<'cx>>,
}

/// An override field that modifies a field in a base type.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct OverrideField<'cx> {
    /// Field path.
    pub field_path: FieldPath<'cx>,

    pub required: bool,

    /// Optional documentation for the field.
    pub doc: Doc<'cx>,

    /// An override applied to the field.
    pub ty: Ty<'cx>,
}

/// Rust-compatible primitive types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Primitive {
    String { format: Option<StringFormat> },
    Boolean { constant: Option<bool> },
    Integer { constant: Option<i64> },
    Number,
    Null,
}

/// Well-known string formats used in GitHub schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StringFormat {
    DateTime,
    Uri,
    Uuid,
}
