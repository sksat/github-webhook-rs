use crate::{
    arena::ArenaAllocator,
    intern::{Interned, Str, StrInterner},
    ir::{DefinitionPath, FieldPath, FieldTreeNode, Path, PathInterner, TyInterner, TyKind},
};

/// Arena holding all shared allocations.
#[derive(Default)]
pub struct Arena<'cx> {
    str_interner: StrInterner<'cx>,
    ty_interner: TyInterner<'cx>,
    path_interner: PathInterner<'cx>,
    field_path_arena: ArenaAllocator<FieldTreeNode<'cx>>,
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
    pub fn alloc_field_path(&'cx self, path: FieldTreeNode<'cx>) -> FieldPath<'cx> {
        FieldPath(self.field_path_arena.alloc(path))
    }
}

/// A long-lived object.
///
/// This context must outlive any references to allocated objects.
pub struct Context<'cx> {
    arena: &'cx Arena<'cx>,
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
        Self { arena }
    }
}
