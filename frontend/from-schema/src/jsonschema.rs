//! Abstract Syntax Tree (AST) for JSON Schema Draft-07 definitions,
//! tailored for GitHub Webhook schema consumption.
//!
//! This module provides deserializable Rust structures representing
//! the structural elements of a subset of JSON Schema used in GitHub Webhooks.

use serde::{de::IgnoredAny, Deserialize};
use std::{borrow::Cow, collections::HashMap};

/// The top-level structure for a JSON Schema document.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct JsonSchema<'a> {
    /// The `$schema` version identifier (e.g., `http://json-schema.org/draft-07/schema#`).
    #[serde(rename = "$schema")]
    pub schema: Option<IgnoredAny>,

    #[serde(borrow)]
    /// A map of schema definitions under `definitions`.
    pub definitions: HashMap<&'a str, PropertySchema<'a>>,

    /// The root schema definition. Indicates that a payload type can be one of the specified schemas.
    pub one_of: Vec<RefSchema<'a>>,
}

/// A property schema.
#[derive(Debug, Deserialize)]
pub struct PropertySchema<'a> {
    /// The `$schema` version identifier (e.g., `http://json-schema.org/draft-07/schema#`).
    #[serde(rename = "$schema")]
    pub schema: Option<IgnoredAny>,

    #[serde(borrow)]
    /// Optional title for the schema.
    pub title: Option<Cow<'a, str>>,

    /// Optional description for the schema.
    pub description: Option<Cow<'a, str>>,

    #[serde(flatten)]
    /// The content of the schema, which can be an object, string, integer, etc.
    pub content: SchemaDefinition<'a>,
}

/// A schema definition, which can be a reference, enum, or primitive.
#[derive(Debug)]
pub enum SchemaDefinition<'a> {
    /// A reference to another definition.
    Ref(RefSchema<'a>),

    /// A `oneOf` schema that contains multiple alternative schemas.
    OneOf(OneOfSchema<'a>),

    /// An `allOf` schema that represents an intersection of multiple schemas.
    AllOf(AllOfSchema<'a>),

    /// A schema for an object type. describes the structure of an object.
    Object(Nullable<ObjectSchema<'a>>),

    /// A schema for a string type, which may include an enum or format.
    String(Nullable<StringSchema<'a>>),

    /// A schema for an integer type, which may include a constant value.
    Integer(Nullable<IntegerSchema>),

    /// A schema for a number type.
    Number(Nullable<NumberSchema>),

    /// A schema for a boolean type, which may include a constant value.
    Boolean(Nullable<BooleanSchema>),

    /// A schema for an array type, which describes the items in the array.
    Array(Nullable<ArraySchema<'a>>),

    /// A null schema, which indicates that the property is `null`.
    Null,
}

/// A `$ref` to another schema definition.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefSchema<'a> {
    /// The `$ref` path (usually internal).
    #[serde(rename = "$ref")]
    pub ref_path: &'a str,
}

/// A `oneOf` variant representing multiple alternative `$ref`s.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct OneOfSchema<'a> {
    #[serde(borrow)]
    /// An array of schema references.
    pub one_of: Vec<SchemaDefinition<'a>>,
}

/// A `allOf` variant representing an intersection of multiple schemas.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AllOfSchema<'a> {
    #[serde(borrow)]
    /// An intersection of schemas.
    pub all_of: Intersection<'a>,
}

/// An restricted form of intersection schema.
///
/// Only recognizes the form of `[{"$ref": "#/..", ..}, {"type": "object", ..}]`.
#[derive(Debug)]
pub struct Intersection<'a> {
    /// The base schema that defines the core structure.
    pub ref_schema: RefSchema<'a>,

    /// An extension schema that adds additional properties or constraints.
    ///
    /// We ignore `"tsAdditionalProperties": false` in the schema.
    pub extension: ObjectSchema<'a>,
}

impl<'de: 'a, 'a> Deserialize<'de> for Intersection<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(expecting = "an intersection schema with a base schema and an extension")]
        /// Parses `[{"$ref": "#/..", ..}, {"type": "object", ..}]`.
        struct IntersectionParser<'a>(#[serde(borrow)] (RefSchema<'a>, ObjectSchema<'a>));

        let IntersectionParser((ref_schema, extension)) =
            IntersectionParser::deserialize(deserializer)?;
        Ok(Self {
            ref_schema,
            extension,
        })
    }
}

/// A schema that can be `null`.
///
/// Note that this struct is not [`Deserialize`]. This is a thin wrapper around
/// the content schema.
#[derive(Debug)]
pub struct Nullable<Content> {
    /// Whether this schema can be `null`.
    pub nullable: bool,

    /// The content of the schema, which can be an object, string, integer, etc.
    pub content: Content,
}

impl<Content> Nullable<Content> {
    pub fn new(nullable: bool, content: Content) -> Self {
        Self { nullable, content }
    }
}

/// A schema representing a JSON object structure.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectSchema<'a> {
    /// Optional `$schema` version identifier. We will ignore this field.
    #[serde(rename = "$schema")]
    pub schema: Option<&'a str>,

    /// Optional schema title.
    pub title: Option<Cow<'a, str>>,

    /// Field/property definitions.
    #[serde(default)]
    pub properties: HashMap<&'a str, PropertySchema<'a>>,

    /// List of required field names.
    #[serde(default)]
    pub required: Vec<&'a str>,

    /// Whether additional properties are allowed.
    #[serde(default)]
    pub additional_properties: AdditionalProperties<'a>,
}

/// Additional properties allowed in the object schema.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AdditionalProperties<'a> {
    /// Whether additional properties are allowed.
    /// If `true`, any additional properties are allowed.
    Boolean(bool),
    /// Additional properties are allowed with a specific type.
    Type(#[serde(borrow)] Box<SchemaDefinition<'a>>),
}

impl Default for AdditionalProperties<'_> {
    fn default() -> Self {
        // By default, additional properties are not allowed.
        AdditionalProperties::Boolean(false)
    }
}

/// Manual `Deserialize` implementation for `SchemaDefinition`.
///
/// This is the primal part of the deserialization logic.
/// Mainly,
impl<'de: 'a, 'a> Deserialize<'de> for SchemaDefinition<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // We use serde's internal implementation of untagged enum deserialization
        // to handle the various schema types.

        // Justification for using private API:
        // We need to use "untagged" functionality to deserialize schema definitions
        // that can be either a reference, oneOf, or a primitive schema.
        // The public API does offer this functionality, but it reports meaningless errors
        // because it does not know about the enum variants.
        // The former three variants are easily deserialized, but the latter
        // requires a custom deserializer to handle the internally tagged enum, so
        // their error messages are precious for debugging.
        let content = <serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
        let deserializer = serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
        if let Ok(ok) = Result::map(
            <RefSchema as Deserialize>::deserialize(deserializer),
            SchemaDefinition::Ref,
        ) {
            return Ok(ok);
        }
        if let Ok(ok) = Result::map(
            <OneOfSchema as Deserialize>::deserialize(deserializer),
            SchemaDefinition::OneOf,
        ) {
            return Ok(ok);
        }
        if let Ok(ok) = Result::map(
            <AllOfSchema as Deserialize>::deserialize(deserializer),
            SchemaDefinition::AllOf,
        ) {
            return Ok(ok);
        }

        // Below, we use serde's internal implementation of internally tagged enum deserialization
        // with some modifications to handle the custom tag value.
        enum Field {
            NullableOrNot {
                nonnull: NonNullKind,
                nullable: bool,
            },
            Null,
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        enum NonNullKind {
            Object,
            String,
            Integer,
            Number,
            Boolean,
            Array,
        }

        // Modification #1:
        // we use `FieldParser` for parsing the `Field` instead of `FieldVisitor`.
        #[derive(Deserialize)]
        enum NullType {
            #[serde(rename = "null")]
            Null,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum FieldParser {
            /// Parses a string array.
            Array2((NonNullKind, NullType)),

            /// Also parses a string array.
            Array1((NullType,)),

            /// Parses a single string.
            String(NonNullKind),

            /// Also parses a single string.
            Null(NullType),
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(match FieldParser::deserialize(deserializer)? {
                    FieldParser::Array2((non_null_kind, _null_type)) => Self::NullableOrNot {
                        nonnull: non_null_kind,
                        nullable: true,
                    },
                    FieldParser::String(non_null_kind) => Self::NullableOrNot {
                        nonnull: non_null_kind,
                        nullable: false,
                    },
                    FieldParser::Array1(_null_type) => Self::Null,
                    FieldParser::Null(_null_type) => Self::Null,
                })
            }
        }
        // Justification for using private API:
        // we need to deserialize an internally tagged enum with a custom tag value.
        // This is not directly supported by serde's public API, so we use the private API.
        let (tag, content) = serde::de::Deserializer::deserialize_any(
            deserializer,
            serde::__private::de::TaggedContentVisitor::<Field>::new(
                "type",
                "internally tagged enum Message'",
            ),
        )?;
        let deserializer = serde::__private::de::ContentDeserializer::new(content);
        match tag {
            Field::NullableOrNot { nonnull, nullable } => {
                // Modification #2:
                // parse the structure based on the non-null kind
                match nonnull {
                    NonNullKind::Object => {
                        let value = ObjectSchema::deserialize(deserializer)?;
                        Ok(SchemaDefinition::Object(Nullable::new(nullable, value)))
                    }
                    NonNullKind::String => {
                        let value = StringSchema::deserialize(deserializer)?;
                        Ok(SchemaDefinition::String(Nullable::new(nullable, value)))
                    }
                    NonNullKind::Integer => {
                        let value = IntegerSchema::deserialize(deserializer)?;
                        Ok(SchemaDefinition::Integer(Nullable::new(nullable, value)))
                    }
                    NonNullKind::Number => {
                        let value = NumberSchema::deserialize(deserializer)?;
                        Ok(SchemaDefinition::Number(Nullable::new(nullable, value)))
                    }
                    NonNullKind::Boolean => {
                        let value = BooleanSchema::deserialize(deserializer)?;
                        Ok(SchemaDefinition::Boolean(Nullable::new(nullable, value)))
                    }
                    NonNullKind::Array => {
                        let value = ArraySchema::deserialize(deserializer)?;
                        Ok(SchemaDefinition::Array(Nullable::new(nullable, value)))
                    }
                }
            }
            Field::Null => Ok(SchemaDefinition::Null),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StringSchema<'a> {
    Enum {
        /// Possible values for this string. Typically this is a tag like `action`.
        ///
        /// The content of this field is an option of a string so that we can handle
        /// nullable strings with this type. It makes this type to adhere to the
        /// [`Nullable`] type, but it can utilize null-pointer optimization.
        #[serde(rename = "enum")]
        #[serde(borrow)]
        enum_values: Vec<Option<&'a str>>,
    },
    Format {
        /// The format of the string, e.g., "date-time", "uri".
        format: StringFormat,
    },

    /// A generic string. This is used when no specific format or enum is defined.
    ///
    /// This field is equivalent to `()`, but serde distinguishes it from empty struct variants.
    Generic {},
}

/// The format of a string, which can be one of several predefined formats.
#[derive(Debug, Deserialize)]
pub enum StringFormat {
    /// A date-time string in ISO 8601 format.
    #[serde(rename = "date-time")]
    DateTime,

    /// A URI string.
    #[serde(rename = "uri")]
    Uri,

    /// A UUID string.
    #[serde(rename = "uuid")]
    Uuid,
}

/// A number schema, which can be used for both integers and floats.
#[derive(Debug, Deserialize)]
pub struct NumberSchema {}

/// An integer schema, which may include a constant value.
#[derive(Debug, Deserialize)]
pub struct IntegerSchema {
    /// Constant value for this integer.
    #[serde(rename = "const")]
    pub constant: Option<i64>,
}

/// A boolean schema, which may include a constant value.
#[derive(Debug)]
pub struct BooleanSchema {
    /// Constant value for this boolean.
    pub constant: Option<bool>,
}

/// A schema for an array, which describes the items in the array.
#[derive(Debug, Deserialize)]
pub struct ArraySchema<'a> {
    #[serde(borrow)]
    /// The type of items in the array.
    pub items: ArrayType<'a>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ArrayType<'a> {
    #[serde(borrow)]
    /// Concrete type for items in the array.
    Specified(Box<SchemaDefinition<'a>>),

    /// A schema that allows any type of items in the array. There is a corner case
    /// that uses this variant: `"rubygems_metadata": { "type": "array", "items": {} }`.
    Any {},
}

/// Manual `Deserialize` implementation for `BooleanSchema`.
///
/// This is necessary because the `const` and `enum` fields
/// can be used interchangeably in the JSON Schema.
impl<'de> Deserialize<'de> for BooleanSchema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum BooleanSchemaParser {
            Const {
                #[serde(rename = "const")]
                constant: Option<bool>,
            },
            Enum {
                #[serde(rename = "enum")]
                enum_value: [bool; 1],
            },
        }

        let internal = BooleanSchemaParser::deserialize(deserializer)?;
        Ok(match internal {
            BooleanSchemaParser::Const { constant } => BooleanSchema { constant },
            BooleanSchemaParser::Enum { enum_value } => BooleanSchema {
                constant: Some(enum_value[0]),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_object() {
        let json = r##"{
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
}"##;

        let deser = &mut serde_json::Deserializer::from_str(json);
        let _schema: PropertySchema =
            serde_path_to_error::deserialize(deser).expect("Failed to parse JSON Schema");
    }

    #[test]
    fn test_parse_string() {
        let json = r##"{ "type": "string" }"##;

        let deser = &mut serde_json::Deserializer::from_str(json);
        let _schema: PropertySchema =
            serde_path_to_error::deserialize(deser).expect("Failed to parse JSON Schema");
    }

    #[test]
    fn test_parse_nullable_string() {
        let json = r##"{ "type": ["string", "null"] }"##;

        let deser = &mut serde_json::Deserializer::from_str(json);
        let _schema: SchemaDefinition =
            serde_path_to_error::deserialize(deser).expect("Failed to parse JSON Schema");
    }

    #[test]
    fn test_parse_allof() {
        let json = r##"{
    "allOf": [
        { "$ref": "#/definitions/alert-instance" },
        {
            "type": "object",
            "required": ["state"],
            "properties": {
                "state": { "type": "string", "enum": ["dismissed"] }
            },
            "tsAdditionalProperties": false
        }
    ]
}"##;

        let deser = &mut serde_json::Deserializer::from_str(json);
        let _schema: PropertySchema =
            serde_path_to_error::deserialize(deser).expect("Failed to parse JSON Schema");
    }

    // #[test]
    // fn test_parse_jsonschema() {
    //     let json = include_str!("../test/full.json");
    //     let deser = &mut serde_json::Deserializer::from_str(json);
    //     let schema: JsonSchema =
    //         serde_path_to_error::deserialize(deser).expect("Failed to parse JSON Schema");
    //     assert!(!schema.definitions.is_empty());
    // }
}
