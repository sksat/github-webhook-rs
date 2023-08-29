use std::{collections::HashMap, marker::PhantomData};

use serde::__private::de::Content;
use serde::de::{self, EnumAccess, MapAccess, SeqAccess};
use serde::{
    de::{DeserializeSeed, Visitor},
    Deserialize, Deserializer,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSchemaRoot<'a> {
    #[serde(flatten)]
    _schema: SchemaVersion,
    #[serde(borrow)]
    pub definitions: HashMap<&'a str, Property<'a>>,
    pub one_of: Vec<RefProperty<'a>>,
}

#[derive(Debug, Deserialize)]
struct SchemaVersion {
    #[serde(rename = "$schema", with = "schema_version")]
    _schema: (),
}

mod schema_version {
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&'de str>::deserialize(deserializer)?;
        if s == "http://json-schema.org/draft-07/schema#"
            || s == "http://json-schema.org/draft-07/schema"
        {
            Ok(())
        } else {
            Err(serde::de::Error::custom(format!(
                "unsupported schema version: {s}"
            )))
        }
    }
}

#[derive(Debug)]
pub enum Property<'a> {
    Ref(RefProperty<'a>),
    Value(TypedValue<'a>),
    Enum(EnumProperty<'a>),
}

#[derive(Debug, Deserialize)]
pub struct RefProperty<'a> {
    #[serde(rename = "$ref")]
    pub _ref: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct StringProperty {
    pub format: StringFormat,
}

#[derive(Debug, Deserialize)]
pub enum StringFormat {
    #[serde(rename = "date-time")]
    DateTime,
    #[serde(rename = "uri")]
    Uri,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectProperty<'a> {
    pub properties: HashMap<&'a str, Property<'a>>,
    pub required: Option<Vec<&'a str>>,
    pub title: Option<&'a str>,
    pub additional_properties: bool,
}

#[derive(Debug, Deserialize)]
pub struct ArrayProperty<'a> {
    #[serde(borrow = "'a")]
    pub items: Box<Property<'a>>,
}

#[derive(Debug)]
pub struct EnumProperty<'a> {
    pub members: EnumMembers<'a>,
}

#[derive(Debug)]
pub enum TypedValue<'a> {
    Null,
    Other {
        is_nullable: bool,
        value: NonNullValue<'a>,
    },
}

#[derive(Debug)]
pub enum NonNullValue<'a> {
    Boolean,
    Integer,
    Number,
    Object(ObjectProperty<'a>),
    Array(ArrayProperty<'a>),
    String(StringProperty),
}

#[derive(Debug)]
pub enum EnumMembers<'a> {
    Str {
        contains_null: bool,
        members: Vec<&'a str>,
    },
    Boolean(Vec<bool>),
}

struct PropertyVisitor<'a> {
    value: PhantomData<Property<'a>>,
}

impl<'a> PropertyVisitor<'a> {
    fn new() -> Self {
        Self { value: PhantomData }
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Property<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (tag, content) = deserializer.deserialize_any(PropertyVisitor::new())?;
        let deserializer = serde::__private::de::ContentDeserializer::<D::Error>::new(content);
        match tag {
            Tag::Ref => RefProperty::deserialize(deserializer).map(Property::Ref),
            Tag::Type(ty) => {
                let (ty, is_nullable) = match ty {
                    PropertyType::Null => return Ok(Property::Value(TypedValue::Null)),
                    PropertyType::NonNull(ty) => (ty, false),
                    PropertyType::Nullable(ty) => (ty, true),
                };
                match ty {
                    NonNullPropertyType::Boolean => Ok(Property::Value(TypedValue::Other {
                        is_nullable,
                        value: NonNullValue::Boolean,
                    })),
                    NonNullPropertyType::Integer => Ok(Property::Value(TypedValue::Other {
                        is_nullable,
                        value: NonNullValue::Integer,
                    })),
                    NonNullPropertyType::Number => Ok(Property::Value(TypedValue::Other {
                        is_nullable,
                        value: NonNullValue::Number,
                    })),
                    NonNullPropertyType::String => {
                        StringProperty::deserialize(deserializer).map(|c| {
                            Property::Value(TypedValue::Other {
                                is_nullable,
                                value: NonNullValue::String(c),
                            })
                        })
                    }
                    NonNullPropertyType::Object => {
                        ObjectProperty::deserialize(deserializer).map(|c| {
                            Property::Value(TypedValue::Other {
                                is_nullable,
                                value: NonNullValue::Object(c),
                            })
                        })
                    }
                    NonNullPropertyType::Array => {
                        ArrayProperty::deserialize(deserializer).map(|c| {
                            Property::Value(TypedValue::Other {
                                is_nullable,
                                value: NonNullValue::Array(c),
                            })
                        })
                    }
                }
            }
            Tag::Enum(ty) => {
                struct StringEnum<'a, const NULLABLE: bool> {
                    value: Vec<&'a str>,
                }
                struct StringEnumVisitor<'a, const NULLABLE: bool> {
                    value: PhantomData<Vec<&'a str>>,
                }
                impl<'a, const NULLABLE: bool> StringEnumVisitor<'a, NULLABLE> {
                    fn new() -> Self {
                        Self { value: PhantomData }
                    }
                }
                impl<'de: 'a, 'a, const NULLABLE: bool> Visitor<'de> for StringEnumVisitor<'a, NULLABLE> {
                    type Value = StringEnum<'a, NULLABLE>;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        if NULLABLE {
                            formatter.write_str("a nullable string enum")
                        } else {
                            formatter.write_str("a non-null string enum")
                        }
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: SeqAccess<'de>,
                    {
                        let mut contains_null = false;
                        let mut vec =
                            Vec::<&'a str>::with_capacity(seq.size_hint().unwrap_or_default());
                        if NULLABLE {
                            if let Some(s) = seq.next_element::<Option<&'a str>>()? {
                                if let Some(s) = s {
                                    vec.push(s);
                                } else {
                                    contains_null = true;
                                }
                            }
                        } else {
                            while let Some(s) = seq.next_element::<&'a str>()? {
                                vec.push(s);
                            }
                        }
                        assert_eq!(NULLABLE, contains_null);
                        Ok(StringEnum { value: vec })
                    }
                }

                impl<'de: 'a, 'a, const NULLABLE: bool> Deserialize<'de> for StringEnum<'a, NULLABLE> {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        deserializer.deserialize_seq(StringEnumVisitor::<NULLABLE>::new())
                    }
                }
                match ty {
                    PropertyType::NonNull(NonNullPropertyType::Boolean) => {
                        #[derive(Deserialize)]
                        struct BooleanEnum {
                            #[serde(rename = "enum")]
                            _enum: Vec<bool>,
                        }
                        BooleanEnum::deserialize(deserializer).map(|c| {
                            Property::Enum(EnumProperty {
                                members: EnumMembers::Boolean(c._enum),
                            })
                        })
                    }
                    PropertyType::NonNull(NonNullPropertyType::String) => {
                        #[derive(Deserialize)]
                        struct NonNullStringEnum<'a> {
                            #[serde(rename = "enum", borrow = "'a")]
                            _enum: StringEnum<'a, false>,
                        }
                        NonNullStringEnum::deserialize(deserializer).map(|c| {
                            Property::Enum(EnumProperty {
                                members: EnumMembers::Str {
                                    contains_null: false,
                                    members: c._enum.value,
                                },
                            })
                        })
                    }
                    PropertyType::Nullable(NonNullPropertyType::String) => {
                        #[derive(Deserialize)]
                        struct NullableStringEnum<'a> {
                            #[serde(rename = "enum", borrow = "'a")]
                            _enum: StringEnum<'a, true>,
                        }
                        NullableStringEnum::deserialize(deserializer).map(|c| {
                            Property::Enum(EnumProperty {
                                members: EnumMembers::Str {
                                    contains_null: true,
                                    members: c._enum.value,
                                },
                            })
                        })
                    }
                    ty => Err(de::Error::custom(format!(
                        "enum of type `{ty}` is not supported"
                    )))?,
                }
            }
        }
    }
}

impl<'de> Visitor<'de> for PropertyVisitor<'de> {
    type Value = (Tag, Content<'de>);

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a property declaration")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut tag_ref: Option<Content> = None;
        let mut tag_type: Option<PropertyType> = None;
        let mut tag_enum: Option<Content> = None;
        let mut vec = Vec::<(Content, Content)>::with_capacity(map.size_hint().unwrap_or_default());
        while let Some(k) = (map.next_key_seed(TagOrContentVisitor::new(
            TagKind::ALL.iter().map(|t| (t.as_str(), *t)).collect(),
        )))? {
            match k {
                TagOrContent::Tag(new_tag) => match new_tag {
                    TagKind::Ref => {
                        if tag_ref.is_some() {
                            return Err(de::Error::duplicate_field("$ref"));
                        }
                        tag_ref = Some(map.next_value()?);
                    }
                    TagKind::Type => {
                        if tag_type.is_some() {
                            return Err(de::Error::duplicate_field("type"));
                        }
                        tag_type = Some(map.next_value()?);
                    }
                    TagKind::Enum => {
                        if tag_enum.is_some() {
                            return Err(de::Error::duplicate_field("enum"));
                        }
                        tag_enum = Some(map.next_value()?);
                    }
                },
                TagOrContent::Content(k) => {
                    let v = map.next_value()?;
                    vec.push((k, v));
                }
            }
        }
        if let Some(ref_value) = tag_ref {
            vec.push((Content::String(TagKind::Ref.as_str().into()), ref_value));
            return Ok((Tag::Ref, Content::Map(vec)));
        }
        if let Some(enum_value) = tag_enum {
            if let Some(type_value) = tag_type {
                vec.push((Content::String(TagKind::Enum.as_str().into()), enum_value));
                return Ok((Tag::Enum(type_value), Content::Map(vec)));
            } else {
                return Err(de::Error::missing_field("type"));
            }
        }
        if let Some(type_value) = tag_type {
            return Ok((Tag::Type(type_value), Content::Map(vec)));
        }
        Err(de::Error::custom("missing field `type`, `enum`, or `$ref`"))
    }
}

struct ContentVisitor<'de> {
    value: PhantomData<Content<'de>>,
}

impl<'de> ContentVisitor<'de> {
    fn new() -> Self {
        ContentVisitor { value: PhantomData }
    }
}

impl<'de> Visitor<'de> for ContentVisitor<'de> {
    type Value = Content<'de>;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("any value")
    }

    fn visit_bool<F>(self, value: bool) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Bool(value))
    }

    fn visit_i64<F>(self, value: i64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::I64(value))
    }

    fn visit_u64<F>(self, value: u64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::U64(value))
    }

    fn visit_f64<F>(self, value: f64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::F64(value))
    }

    fn visit_str<F>(self, value: &str) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::String(value.into()))
    }

    fn visit_bytes<F>(self, value: &[u8]) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::ByteBuf(value.into()))
    }

    fn visit_unit<F>(self) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::Unit)
    }

    fn visit_none<F>(self) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Content::None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|v| Content::Some(Box::new(v)))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|v| Content::Newtype(Box::new(v)))
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut vec = Vec::<Content>::with_capacity(visitor.size_hint().unwrap_or(0));
        while let Some(e) = visitor.next_element()? {
            vec.push(e);
        }
        Ok(Content::Seq(vec))
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut vec =
            Vec::<(Content, Content)>::with_capacity(visitor.size_hint().unwrap_or_default());
        while let Some(kv) = visitor.next_entry()? {
            vec.push(kv);
        }
        Ok(Content::Map(vec))
    }

    fn visit_enum<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
    where
        V: EnumAccess<'de>,
    {
        Err(de::Error::custom(
            "untagged and internally tagged enums do not support enum input",
        ))
    }
}

#[derive(Debug)]
enum Tag {
    Ref,
    Type(PropertyType),
    Enum(PropertyType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TagKind {
    Ref,
    Type,
    Enum,
}

impl TagKind {
    const REF: &'static str = "$ref";
    const TYPE: &'static str = "type";
    const ENUM: &'static str = "enum";
    const ALL: [Self; 3] = [Self::Ref, Self::Type, Self::Enum];
    fn as_str(&self) -> &'static str {
        match self {
            TagKind::Ref => Self::REF,
            TagKind::Type => Self::TYPE,
            TagKind::Enum => Self::ENUM,
        }
    }
}

enum TagOrContent<'a> {
    Tag(TagKind),
    Content(Content<'a>),
}

struct TagOrContentVisitor<'a> {
    names: HashMap<&'a str, TagKind>,
}

impl<'a> TagOrContentVisitor<'a> {
    fn new(names: HashMap<&'a str, TagKind>) -> Self {
        Self { names }
    }
}

impl<'de> Visitor<'de> for TagOrContentVisitor<'de> {
    type Value = TagOrContent<'de>;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "a type tag or any other value")
    }

    fn visit_bool<F>(self, value: bool) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new()
            .visit_bool(value)
            .map(TagOrContent::Content)
    }

    fn visit_i64<F>(self, value: i64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new()
            .visit_i64(value)
            .map(TagOrContent::Content)
    }

    fn visit_u64<F>(self, value: u64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new()
            .visit_u64(value)
            .map(TagOrContent::Content)
    }

    fn visit_f64<F>(self, value: f64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new()
            .visit_f64(value)
            .map(TagOrContent::Content)
    }

    fn visit_str<F>(self, value: &str) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        if let Some(t) = self.names.get(value) {
            Ok(TagOrContent::Tag(*t))
        } else {
            ContentVisitor::new()
                .visit_str(value)
                .map(TagOrContent::Content)
        }
    }

    fn visit_bytes<F>(self, value: &[u8]) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        if let Some((_, t)) = self.names.iter().find(|(k, _)| k.as_bytes() == value) {
            Ok(TagOrContent::Tag(*t))
        } else {
            ContentVisitor::new()
                .visit_bytes(value)
                .map(TagOrContent::Content)
        }
    }

    fn visit_unit<F>(self) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new()
            .visit_unit()
            .map(TagOrContent::Content)
    }

    fn visit_none<F>(self) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new()
            .visit_none()
            .map(TagOrContent::Content)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        ContentVisitor::new()
            .visit_some(deserializer)
            .map(TagOrContent::Content)
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        ContentVisitor::new()
            .visit_newtype_struct(deserializer)
            .map(TagOrContent::Content)
    }

    fn visit_seq<V>(self, visitor: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        ContentVisitor::new()
            .visit_seq(visitor)
            .map(TagOrContent::Content)
    }

    fn visit_map<V>(self, visitor: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        ContentVisitor::new()
            .visit_map(visitor)
            .map(TagOrContent::Content)
    }

    fn visit_enum<V>(self, visitor: V) -> Result<Self::Value, V::Error>
    where
        V: EnumAccess<'de>,
    {
        ContentVisitor::new()
            .visit_enum(visitor)
            .map(TagOrContent::Content)
    }
}

impl<'de> DeserializeSeed<'de> for TagOrContentVisitor<'de> {
    type Value = TagOrContent<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        // self-describing format
        deserializer.deserialize_any(self)
    }
}

#[derive(Debug)]
enum PropertyType {
    Null,
    NonNull(NonNullPropertyType),
    Nullable(NonNullPropertyType),
}

impl std::fmt::Display for PropertyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyType::Null => write!(f, "null"),
            PropertyType::NonNull(ty) => write!(f, "{}", ty.ts_type_str()),
            PropertyType::Nullable(ty) => write!(f, "{} | null", ty.ts_type_str()),
        }
    }
}

impl From<PropertyTypeConversion> for PropertyType {
    fn from(value: PropertyTypeConversion) -> Self {
        match value {
            PropertyTypeConversion::Null => Self::Null,
            PropertyTypeConversion::NonNull(ty) => Self::NonNull(ty),
        }
    }
}

#[derive(Debug)]
enum NonNullPropertyType {
    Boolean,
    Integer,
    Object,
    Array,
    Number,
    String,
}

impl NonNullPropertyType {
    fn ts_type_str(&self) -> &'static str {
        match self {
            NonNullPropertyType::Boolean => "boolean",
            NonNullPropertyType::Integer => "number",
            NonNullPropertyType::Object => "object",
            NonNullPropertyType::Array => "array",
            NonNullPropertyType::Number => "number",
            NonNullPropertyType::String => "string",
        }
    }
}

#[derive(Debug)]
enum PropertyTypeConversion {
    Null,
    NonNull(NonNullPropertyType),
}

#[derive(Debug)]
enum PropertyTypeConversionError {
    UnsupportedType,
}

impl NonNullPropertyType {
    fn try_from_str(s: &str) -> Result<PropertyTypeConversion, PropertyTypeConversionError> {
        Ok(PropertyTypeConversion::NonNull(match s {
            "null" => return Ok(PropertyTypeConversion::Null),
            "boolean" => Self::Boolean,
            "integer" => Self::Integer,
            "object" => Self::Object,
            "array" => Self::Array,
            "number" => Self::Number,
            "string" => Self::String,
            _ => Err(PropertyTypeConversionError::UnsupportedType)?,
        }))
    }
}

struct PropertyTypeVisitor;

impl<'de> Visitor<'de> for PropertyTypeVisitor {
    type Value = PropertyType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a property type")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let new_ty = NonNullPropertyType::try_from_str(s).map_err(|e| match e {
            PropertyTypeConversionError::UnsupportedType => {
                de::Error::custom(format!("unsupported property type: `{s}`"))
            }
        })?;
        Ok(match new_ty {
            PropertyTypeConversion::Null => PropertyType::Null,
            PropertyTypeConversion::NonNull(ty) => PropertyType::NonNull(ty),
        })
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut ty = None;
        while let Some(s) = seq.next_element::<&str>()? {
            let new_ty = NonNullPropertyType::try_from_str(s).map_err(|e| match e {
                PropertyTypeConversionError::UnsupportedType => {
                    de::Error::custom(format!("unsupported property type: `{s}`"))
                }
            })?;
            if let Some(old_ty) = ty {
                match (old_ty, new_ty) {
                    (PropertyType::Null, PropertyTypeConversion::NonNull(nn))
                    | (PropertyType::NonNull(nn), PropertyTypeConversion::Null) => {
                        ty = Some(PropertyType::Nullable(nn));
                    }
                    (old_ty, new_ty) => {
                        return Err(de::Error::custom(
                            format!("unimplemented combination of property types (bug): `{old_ty:?}` and `{new_ty:?}`"),
                        ));
                    }
                };
            } else {
                ty = Some(new_ty.into());
            }
        }
        match ty {
            None => Err(de::Error::custom("missing property `type`")),
            Some(ty) => Ok(ty),
        }
    }
}

impl<'de> Deserialize<'de> for PropertyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(PropertyTypeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_version() {
        let s = r#"
        {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {
                "foo": {
                    "type": "string"
                }
            }
        }
        "#;
        let _schema: SchemaVersion = serde_json::from_str(s).unwrap();
    }
    #[test]
    fn test_schema() {
        let s = r##"
        {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "definitions": {
                "branch_protection_rule$created": {
                    "$schema": "http://json-schema.org/draft-07/schema",
                    "type": "object",
                    "description": "Activity related to a branch protection rule. For more information, see \"[About branch protection rules](https://docs.github.com/en/github/administering-a-repository/defining-the-mergeability-of-pull-requests/about-protected-branches#about-branch-protection-rules).\"",
                    "required": ["action", "rule", "repository", "sender"],
                    "properties": {
                        "action": { "type": "string", "enum": ["created"] },
                        "rule": { "$ref": "#/definitions/branch-protection-rule" },
                        "repository": { "$ref": "#/definitions/repository" },
                        "sender": { "$ref": "#/definitions/user" },
                        "installation": { "$ref": "#/definitions/installation-lite" },
                        "organization": { "$ref": "#/definitions/organization" }
                    },
                    "title": "branch protection rule created event",
                    "additionalProperties": false
                }
            },
            "oneOf": [
                { "$ref": "#/definitions/branch_protection_rule_event" }
            ]
        }"##;
        let _schema: JsonSchemaRoot = serde_json::from_str(s).unwrap();
        dbg!(_schema);
    }
}
