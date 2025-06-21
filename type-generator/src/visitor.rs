use crate::ir::{
    Additional, ConstEnum, ConstEnumVariant, Definition, Doc, Field, FieldPath, Module, Override,
    OverrideToField, Overrides, Path, Primitive, Struct, TaggedEnum, TaggedEnumVariant, Ty, TyKind,
    UntaggedEnum,
};

/// Distinguishes between the context in which a path appears.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathSite {
    /// Path is used in a type position.
    Use,
    /// Path is used to define a type.
    Def,
}

/// Trait for recursively walking over IR elements.
///
/// Users can override `visit_*` methods to customize traversal.
/// The default implementations call `super_*`, which perform structural traversal.
pub trait Visitor<'cx> {
    fn visit_module(&mut self, module: &Module<'cx>) {
        self.super_module(module);
    }

    fn super_module(&mut self, module: &Module<'cx>) {
        let Module {
            definitions,
            variants,
        } = module;
        self.visit_definitions(definitions);
        self.visit_toplevel_variants(variants);
    }

    fn visit_toplevel_variants(&mut self, variants: &[Path<'cx>]) {
        self.super_toplevel_variants(variants);
    }

    fn super_toplevel_variants(&mut self, variants: &[Path<'cx>]) {
        for variant in variants {
            self.visit_toplevel_variant(variant);
        }
    }

    fn visit_toplevel_variant(&mut self, variant: &Path<'cx>) {
        self.super_toplevel_variant(variant);
    }

    fn super_toplevel_variant(&mut self, variant: &Path<'cx>) {
        self.visit_path(*variant, PathSite::Use);
    }

    fn visit_definitions(&mut self, definitions: &[Definition<'cx>]) {
        self.super_definitions(definitions);
    }

    fn super_definitions(&mut self, definitions: &[Definition<'cx>]) {
        for definition in definitions {
            self.visit_definition(definition);
        }
    }

    fn visit_definition(&mut self, definition: &Definition<'cx>) {
        self.super_definition(definition);
    }

    fn super_definition(&mut self, definition: &Definition<'cx>) {
        let Definition { doc, ty } = definition;
        self.visit_doc(doc);
        self.visit_ty(*ty);
    }

    fn visit_doc(&mut self, doc: &Doc<'cx>) {
        self.super_doc(doc);
    }

    fn super_doc(&mut self, _doc: &Doc<'cx>) {}

    fn visit_ty(&mut self, ty: Ty<'cx>) {
        self.super_ty(ty);
    }

    fn super_ty(&mut self, ty: Ty<'cx>) {
        let Ty { path, kind } = ty;
        self.visit_path(path, PathSite::Use);
        self.visit_ty_kind(*kind);
    }

    fn visit_ty_kind(&mut self, kind: &TyKind<'cx>) {
        self.super_ty_kind(kind);
    }

    fn super_ty_kind(&mut self, kind: &TyKind<'cx>) {
        match kind {
            TyKind::Ident(path) => self.visit_path(*path, PathSite::Use),
            TyKind::ConstEnum(const_enum) => self.visit_const_enum(const_enum),
            TyKind::TaggedEnum(tagged_enum) => self.visit_tagged_enum(tagged_enum),
            TyKind::UntaggedEnum(untagged_enum) => self.visit_untagged_enum(untagged_enum),
            TyKind::Struct(s) => self.visit_struct_ty(s),
            TyKind::Override(o) => self.visit_override(o),
            TyKind::Primitive(p) => self.visit_primitive(*p),
            TyKind::Option(ty) => self.visit_option(*ty),
            TyKind::Vec(ty) => self.visit_vec(*ty),
            TyKind::AnyVec => {}
        }
    }

    fn visit_const_enum(&mut self, const_enum: &ConstEnum<'cx>) {
        self.super_const_enum(const_enum);
    }

    fn super_const_enum(&mut self, const_enum: &ConstEnum<'cx>) {
        let ConstEnum { variants } = const_enum;
        for variant in variants {
            self.visit_const_enum_variant(variant);
        }
    }

    fn visit_const_enum_variant(&mut self, variant: &ConstEnumVariant<'cx>) {
        self.super_const_enum_variant(variant);
    }

    fn super_const_enum_variant(&mut self, _variant: &ConstEnumVariant<'cx>) {}

    fn visit_tagged_enum(&mut self, tagged_enum: &TaggedEnum<'cx>) {
        self.super_tagged_enum(tagged_enum);
    }

    fn super_tagged_enum(&mut self, tagged_enum: &TaggedEnum<'cx>) {
        let TaggedEnum { variants } = tagged_enum;
        for variant in variants {
            self.visit_tagged_enum_variant(variant);
        }
    }

    fn visit_tagged_enum_variant(&mut self, variant: &TaggedEnumVariant<'cx>) {
        self.super_tagged_enum_variant(variant);
    }

    fn super_tagged_enum_variant(&mut self, _variant: &TaggedEnumVariant<'cx>) {}

    fn visit_untagged_enum(&mut self, untagged_enum: &UntaggedEnum<'cx>) {
        self.super_untagged_enum(untagged_enum);
    }

    fn super_untagged_enum(&mut self, untagged_enum: &UntaggedEnum<'cx>) {
        let UntaggedEnum { variants } = untagged_enum;
        for variant in variants {
            self.visit_ty(*variant);
        }
    }

    fn visit_struct_ty(&mut self, s: &Struct<'cx>) {
        self.super_struct_ty(s);
    }

    fn super_struct_ty(&mut self, s: &Struct<'cx>) {
        let Struct { fields, additional } = s;
        for field in fields {
            self.visit_field(field);
        }
        self.visit_additional(*additional);
    }

    fn visit_additional(&mut self, additional: Additional<'cx>) {
        self.super_additional(additional);
    }

    fn super_additional(&mut self, additional: Additional<'cx>) {
        match additional {
            Additional::None => {}
            Additional::Typed(ty) => {
                self.visit_ty(ty);
            }
            Additional::Any => {}
        }
    }

    fn visit_field(&mut self, field: &Field<'cx>) {
        self.super_field(field);
    }

    fn super_field(&mut self, field: &Field<'cx>) {
        let Field {
            name: _,
            ty,
            required: _,
            doc,
        } = field;
        self.visit_ty(*ty);
        self.visit_doc(doc);
    }

    fn visit_override(&mut self, o: &Override<'cx>) {
        self.super_override(o);
    }

    fn super_override(&mut self, o: &Override<'cx>) {
        let Override { base_ty, fields } = o;
        self.visit_path(*base_ty, PathSite::Use);
        for field in fields {
            self.visit_override_field(field);
        }
    }

    fn visit_override_field(&mut self, field: &Overrides<'cx>) {
        self.super_override_field(field);
    }

    fn super_override_field(&mut self, field: &Overrides<'cx>) {
        let Overrides {
            field_path,
            content:
                OverrideToField {
                    required: _,
                    doc,
                    ty,
                },
        } = field;
        self.visit_field_path(*field_path);
        self.visit_doc(doc);
        self.visit_ty(*ty);
    }

    fn visit_field_path(&mut self, path: FieldPath<'cx>) {
        self.super_field_path(path);
    }

    fn super_field_path(&mut self, _path: FieldPath<'cx>) {}

    fn visit_path(&mut self, path: Path<'cx>, site: PathSite) {
        self.super_path(path, site);
    }

    fn super_path(&mut self, _path: Path<'cx>, _site: PathSite) {}

    fn visit_primitive(&mut self, prim: Primitive) {
        self.super_primitive(prim);
    }

    fn super_primitive(&mut self, _prim: Primitive) {}

    fn visit_option(&mut self, ty: Ty<'cx>) {
        self.super_option(ty);
    }

    fn super_option(&mut self, ty: Ty<'cx>) {
        self.visit_ty(ty);
    }

    fn visit_vec(&mut self, ty: Ty<'cx>) {
        self.super_vec(ty);
    }

    fn super_vec(&mut self, ty: Ty<'cx>) {
        self.visit_ty(ty);
    }
}
