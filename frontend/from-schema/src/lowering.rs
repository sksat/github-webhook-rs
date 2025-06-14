//! Conversion from the `jsonschema` AST into the IR (`ir::Toplevel`, `ir::Ty`, etc.)
//! in small, testable steps.  All `lower_*` logic that’s one-to-one with an AST type
//! is implemented as a `Lower` trait; the top-level conversion and other multi-variant
//! cases remain free functions.

use github_webhook_type_generator::{
    context::{self},
    ir::{
        self, Additional, ConstEnumVariant, Definition, DefinitionPath, DefinitionRoot, Doc, Field,
        FieldPath, Module, Override, OverrideField, Path, Primitive, TaggedEnumVariant, Ty, TyKind,
    },
};

use crate::jsonschema::{
    AdditionalProperties, AllOfSchema, ArraySchema, ArrayType, BooleanSchema, IntegerSchema,
    JsonSchema, Nullable, NumberSchema, ObjectSchema, OneOfSchema, PropertySchema, RefSchema,
    SchemaDefinition, StringFormat, StringSchema,
};

struct Context<'cx> {
    gcx: &'cx context::Context<'cx>,
    definitions: Vec<Definition<'cx>>,
}

impl<'cx> std::ops::Deref for Context<'cx> {
    type Target = &'cx context::Context<'cx>;

    fn deref(&self) -> &Self::Target {
        &self.gcx
    }
}

/// Entrypoint: convert the root JSON schema into an IR forest.
pub fn lower_schema<'cx>(schema: JsonSchema<'cx>, gcx: &'cx context::Context<'cx>) -> Module<'cx> {
    let mut cx = Context {
        gcx,
        definitions: Vec::with_capacity(schema.definitions.len()),
    };

    for (raw_key, property) in schema.definitions {
        // 1) compute DefinitionRoot from "foo$bar"
        let root = lower_definition_root(&mut cx, raw_key);

        // 2) start a fresh path
        let path = Path::mk_root(&cx, root);

        // 3) lower this PropertySchema into a `Ty<'cx>`
        let (doc, ty) = property.lower(&mut cx, path);
        cx.definitions.push(Definition { doc, ty });
    }

    let variants = schema
        .one_of
        .into_iter()
        .map(|one_of| lower_ref(&mut cx, &one_of))
        .collect();

    Module {
        definitions: cx.definitions,
        variants,
    }
}

fn lower_definition_root<'cx>(cx: &mut Context<'cx>, key: &'cx str) -> DefinitionRoot<'cx> {
    let (base, variant) = match key.split_once('$') {
        Some((base, variant)) => (cx.intern_str(base), Some(cx.intern_str(variant))),
        None => (cx.intern_str(key), None),
    };
    DefinitionRoot { base, variant }
}

fn lower_ref<'cx>(cx: &mut Context<'cx>, ref_schema: &RefSchema<'cx>) -> Path<'cx> {
    let raw = ref_schema.ref_path;
    let key = raw.strip_prefix("#/definitions/").expect("invalid ref");
    let root = lower_definition_root(cx, key);
    Path::mk_root(cx, root)
}

/// Trait for schema components that lower into a single IR construct.
trait Lower<'cx> {
    type Output;

    fn lower(self, cx: &mut Context<'cx>, path: Path<'cx>) -> Self::Output;
}

impl<'cx> Lower<'cx> for RefSchema<'cx> {
    type Output = Ty<'cx>;

    fn lower(self, cx: &mut Context<'cx>, path: Path<'cx>) -> Self::Output {
        Ty::mk_ident(cx, lower_ref(cx, &self), path)
    }
}

impl<'cx> Lower<'cx> for PropertySchema<'cx> {
    type Output = (Doc<'cx>, Ty<'cx>);

    fn lower(self, cx: &mut Context<'cx>, path: Path<'cx>) -> Self::Output {
        let doc = Doc {
            title: self.title,
            description: self.description,
        };

        let ty = self.content.lower(cx, path);

        (doc, ty)
    }
}

impl<'cx> Lower<'cx> for SchemaDefinition<'cx> {
    type Output = Ty<'cx>;

    fn lower(self, cx: &mut Context<'cx>, path: Path<'cx>) -> Self::Output {
        match self {
            SchemaDefinition::Ref(schema_ref) => schema_ref.lower(cx, path),
            SchemaDefinition::OneOf(one_of) => one_of.lower(cx, path),
            SchemaDefinition::AllOf(all_of) => all_of.lower(cx, path),
            SchemaDefinition::Object(obj) => obj.lower(cx, path),
            SchemaDefinition::String(string_schema) => string_schema.lower(cx, path),
            SchemaDefinition::Integer(integer_schema) => integer_schema.lower(cx, path),
            SchemaDefinition::Number(number_schema) => number_schema.lower(cx, path),
            SchemaDefinition::Boolean(boolean_schema) => boolean_schema.lower(cx, path),
            SchemaDefinition::Array(array_schema) => array_schema.lower(cx, path),
            SchemaDefinition::Null => Ty::mk_null(cx, path),
        }
    }
}

impl<'cx, Inner: Lower<'cx, Output = Ty<'cx>>> Lower<'cx> for Nullable<Inner> {
    type Output = Ty<'cx>;

    fn lower(self, cx: &mut Context<'cx>, path: Path<'cx>) -> Self::Output {
        let inner_ty = self.content.lower(cx, path);
        Ty::mk_option(cx, inner_ty, path)
    }
}

impl<'cx> Lower<'cx> for ObjectSchema<'cx> {
    type Output = Ty<'cx>;

    fn lower(self, cx: &mut Context<'cx>, path: Path<'cx>) -> Self::Output {
        let mut fields = Vec::with_capacity(self.properties.len());

        for (field_name, field_schema) in self.properties {
            // Skipping fields if they are a tag for a tagged enum.
            if field_name == "action" && path.root().variant.is_some() {
                continue;
            }
            let field_name = cx.intern_str(field_name);
            let absolute_field_path = path.descend(cx, field_name);
            let (doc, ty) = field_schema.lower(cx, absolute_field_path);

            let field = Field {
                name: field_name,
                ty,
                required: self.required.contains(&field_name.0),
                doc,
            };

            fields.push(field);
        }

        let additional = match self.additional_properties {
            AdditionalProperties::Boolean(false) => Additional::None,
            AdditionalProperties::Boolean(true) => Additional::Any,
            AdditionalProperties::Type(boxed) => {
                let ty = boxed.lower(cx, path);
                Additional::Typed(ty)
            }
        };

        Ty::mk_struct(cx, fields, additional, path)
    }
}

impl<'cx> Lower<'cx> for StringSchema<'cx> {
    type Output = Ty<'cx>;

    fn lower(self, cx: &mut Context<'cx>, path: Path<'cx>) -> Self::Output {
        match self {
            StringSchema::Enum { enum_values } => {
                let variants = enum_values
                    .iter()
                    .flatten()
                    .map(|value| ConstEnumVariant {
                        value: cx.intern_str(value),
                    })
                    .collect();
                Ty::mk_const_enum(cx, variants, path)
            }
            StringSchema::Format { format } => {
                let format = match format {
                    StringFormat::DateTime => ir::StringFormat::DateTime,
                    StringFormat::Uri => ir::StringFormat::Uri,
                    StringFormat::Uuid => ir::StringFormat::Uuid,
                };
                Ty::mk_string(cx, Some(format), path)
            }
            StringSchema::Generic {} => Ty::mk_string(cx, None, path),
        }
    }
}

impl<'cx> Lower<'cx> for IntegerSchema {
    type Output = Ty<'cx>;

    fn lower(self, cx: &mut Context<'cx>, path: Path<'cx>) -> Self::Output {
        Ty::mk_primitive(
            cx,
            Primitive::Integer {
                constant: self.constant,
            },
            path,
        )
    }
}

impl<'cx> Lower<'cx> for NumberSchema {
    type Output = Ty<'cx>;

    fn lower(self, cx: &mut Context<'cx>, path: Path<'cx>) -> Self::Output {
        Ty::mk_primitive(cx, Primitive::Number, path)
    }
}

impl<'cx> Lower<'cx> for BooleanSchema {
    type Output = Ty<'cx>;

    fn lower(self, cx: &mut Context<'cx>, path: Path<'cx>) -> Self::Output {
        Ty::mk_boolean(cx, self.constant, path)
    }
}

impl<'cx> Lower<'cx> for ArraySchema<'cx> {
    type Output = Ty<'cx>;

    fn lower(self, cx: &mut Context<'cx>, path: Path<'cx>) -> Self::Output {
        match self.items {
            ArrayType::Specified(inner_schema) => {
                let inner_ty = inner_schema.lower(cx, path);
                Ty::mk_vec(cx, inner_ty, path)
            }
            ArrayType::Any {} => {
                // fallback type for 'any' items
                Ty::mk_any_vec(cx, path)
            }
        }
    }
}

impl<'cx> Lower<'cx> for OneOfSchema<'cx> {
    type Output = Ty<'cx>;

    /// Lower a [`OneOfSchema`] into a type.
    ///
    /// This function is a bit more complex because it needs to handle
    /// multiple usages of `OneOfSchema`:
    /// - as a simple nullable type,
    /// - as a tagged union,
    /// - as a regular union.
    fn lower(self, cx: &mut Context<'cx>, path: Path<'cx>) -> Self::Output {
        let possibilities: Vec<_> = self
            .one_of
            .into_iter()
            .map(|schema| schema.lower(cx, path))
            .collect();

        let contains_null = possibilities.iter().any(|ty| ty.is_null());

        fn is_tagged_enum<'cx>(possibilities: &[Ty<'cx>]) -> Option<Vec<TaggedEnumVariant<'cx>>> {
            let mut common_base = None;
            let len = possibilities.len();
            let variants = possibilities.iter().try_fold(
                Vec::with_capacity(len),
                |mut acc, Ty { kind, .. }| {
                    let TyKind::Ident(Path(path)) = kind.0 else {
                        return None;
                    };
                    let DefinitionPath::Root(DefinitionRoot {
                        base,
                        variant: Some(tag_value),
                    }) = path.0
                    else {
                        return None;
                    };
                    // Check if the base is the same for all possibilities.
                    if let Some(common) = common_base {
                        if common != base {
                            return None;
                        }
                    } else {
                        common_base = Some(base);
                    }

                    // All checks passed, we can add the variant.
                    acc.push(TaggedEnumVariant {
                        tag_value: *tag_value,
                    });
                    Some(acc)
                },
            )?;
            // At this point, we have a valid tagged enum.
            // Remove the tag from the defined variants.

            Some(variants)
        }

        if possibilities.is_empty() {
            unreachable!("`oneOf` with no possibilities is not allowed");
        }

        let non_null_count = possibilities.len() - contains_null as usize;

        match non_null_count {
            0 => unreachable!("`oneOf` with no possibilities is not allowed"),
            1 => {
                if contains_null {
                    // Simply wrap the single non-null type in an `Option`.
                    Ty::mk_option(cx, possibilities.into_iter().next().unwrap(), path)
                } else {
                    // If there's only one possibility, we can return it directly.
                    possibilities.into_iter().next().unwrap()
                }
            }
            _ => {
                if let Some(variants) = is_tagged_enum(&possibilities) {
                    Ty::mk_tagged_enum(cx, variants, path)
                } else {
                    // Fallback to an untagged enum, even if it `contains_null`.
                    Ty::mk_untagged_enum(cx, possibilities, path)
                }
            }
        }
    }
}

impl<'cx> Lower<'cx> for AllOfSchema<'cx> {
    type Output = Ty<'cx>;

    ///  GitHub-Webhook schemas employ `allOf` in a *single, restricted* pattern:
    ///
    ///  ```ignore
    ///  ┌─ first element ────────────────────────────────────────────────────────┐
    ///  │ { "$ref": "#/definitions/base_event" }                                 │
    ///  └────────────────────────────────────────────────────────────────────────┘
    ///  ┌─ second element (object) ──────────────────────────────────────────────┐
    ///  │ { "type": "object", "required": [...], "properties": { ... } }         │
    ///  └────────────────────────────────────────────────────────────────────────┘
    ///  ```
    ///
    ///  This function lowers such a construct to an [`Override`] which records:
    ///
    ///  - `base_ty`   – the resolved [`Path`] of the referenced base event type.  
    ///  - `fields`    – every *non-object* leaf inside the extension object,
    ///                  expressed as [`OverrideField`]s.  Each field path starts
    ///                  with `FieldPath::mk_root(cx)` and is extended via `.descend(...)`.
    ///
    ///  The helper `collect_override_fields` performs a depth-first walk and
    ///  generates the field overrides without duplicating decision logic.
    fn lower(self, cx: &mut Context<'cx>, path: Path<'cx>) -> Self::Output {
        /*───────────────────────────────────────────────────────────────────────
        1.  Resolve the base `$ref` into a Path
        ───────────────────────────────────────────────────────────────────────*/
        let base_type_path: Path<'cx> = lower_ref(cx, &self.all_of.ref_schema);

        /*───────────────────────────────────────────────────────────────────────
        2.  Collect all overriding leaves from the extension object
        ───────────────────────────────────────────────────────────────────────*/
        let mut override_fields: Vec<OverrideField<'cx>> = Vec::new();

        collect_override_fields(
            cx,
            self.all_of.extension,
            FieldPath::mk_root(cx),
            &mut override_fields,
            path,
        );

        /// Recursively traverses `object_schema`, creating an [`OverrideField`] for
        /// every leaf that is *not* an `"object"` schema.
        ///
        /// * `current_fp` represents the path from the extension object’s root to the
        ///   current position.
        fn collect_override_fields<'cx>(
            cx: &mut Context<'cx>,
            object_schema: ObjectSchema<'cx>,
            current_fp: FieldPath<'cx>,
            out: &mut Vec<OverrideField<'cx>>,
            parent_ty: Path<'cx>,
        ) {
            let required_set = &object_schema.required;
            for (field_name, field_schema) in object_schema.properties {
                let next_fp = current_fp.descend(cx, cx.intern_str(field_name));

                match field_schema.content {
                    // recurse
                    SchemaDefinition::Object(inner_obj) => {
                        assert!(!inner_obj.nullable);
                        collect_override_fields(cx, inner_obj.content, next_fp, out, parent_ty);
                    }

                    // leaf
                    _ => {
                        let is_required = required_set.iter().any(|r| *r == field_name);

                        let (doc, ty) = field_schema.lower(cx, parent_ty);

                        out.push(OverrideField {
                            field_path: next_fp,
                            required: is_required,
                            doc,
                            ty,
                        });
                    }
                }
            }
        }

        /*───────────────────────────────────────────────────────────────────────
        3.  Build and return the Override
        ───────────────────────────────────────────────────────────────────────*/
        let override_ = Override {
            base_ty: base_type_path,
            fields: override_fields,
        };
        Ty::mk_override(cx, override_, path)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_lower_full() {
    //     let json = include_str!("../test/full.json");
    //     let deser = &mut serde_json::Deserializer::from_str(json);
    //     let schema: JsonSchema =
    //         serde_path_to_error::deserialize(deser).expect("Failed to parse JSON Schema");
    //     let arena = context::Arena::default();
    //     let gcx = context::Context::new(&arena);
    //     let toplevel = lower_schema(schema, &gcx);
    //     assert!(
    //         !toplevel.definitions.is_empty(),
    //         "Expected non-empty definitions"
    //     );
    // }
}
