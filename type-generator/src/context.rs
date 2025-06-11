use crate::{
    intern::{Interned, Str, StrInterner},
    ir::{
        DefinitionFieldPath, DefinitionPath, FieldPath, FieldPathInterner, Path, PathInterner,
        TyInterner, TyKind,
    },
};

/// Arena holding all shared allocations.
#[derive(Default)]
pub struct Arena<'cx> {
    str_interner: StrInterner<'cx>,
    ty_interner: TyInterner<'cx>,
    path_interner: PathInterner<'cx>,
    field_path_interner: FieldPathInterner<'cx>,
}

impl<'cx> Arena<'cx> {
    /// Allocates a new string in the arena.
    pub fn intern_str(&'cx self, s: &'cx str) -> Str<'cx> {
        self.str_interner.intern_ref_noalloc(s)
    }

    /// Allocates a new type in the arena.
    pub fn intern_ty(&'cx self, ty: TyKind<'cx>) -> Interned<TyInterner<'cx>> {
        self.ty_interner.intern(ty)
    }

    /// Allocates a new path in the arena.
    pub fn intern_path(&'cx self, path: DefinitionPath<'cx>) -> Path<'cx> {
        Path(self.path_interner.intern(path))
    }

    /// Allocates a new field path in the arena.
    pub fn intern_field_path(&'cx self, path: DefinitionFieldPath<'cx>) -> FieldPath<'cx> {
        FieldPath(self.field_path_interner.intern(path))
    }
}

/// A long-lived object.
///
/// This context must outlive any references to allocated objects.
pub struct Context<'cx> {
    arena: &'cx Arena<'cx>,
    pub pre_interned: PreInterned<'cx>,
}

impl<'cx> std::ops::Deref for Context<'cx> {
    type Target = &'cx Arena<'cx>;

    fn deref(&self) -> &Self::Target {
        &self.arena
    }
}

impl<'cx> Context<'cx> {
    /// Creates a new context with a root namespace.
    pub fn new(arena: &'cx Arena<'cx>) -> Self {
        Self {
            arena,
            pre_interned: PreInterned {
                root_field_path: arena.intern_field_path(DefinitionFieldPath::Root(())),
            },
        }
    }
}

/// A pre-interned constants.
pub struct PreInterned<'cx> {
    pub root_field_path: FieldPath<'cx>,
}
