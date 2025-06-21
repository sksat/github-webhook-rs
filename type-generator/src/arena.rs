/// Abstraction over [`typed_arena::Arena`].
pub trait ArenaAlloc<'cx, T> {
    /// Allocates a new value in the arena and returns an interned handle.
    fn alloc(&'cx self, value: T) -> &'cx T;
}

/// See [`ArenaAlloc`].
pub trait ArenaRefAlloc<'cx, T: ?Sized> {
    /// Allocates a new value in the arena and returns an interned handle.
    fn alloc(&'cx self, value: &T) -> &'cx T;
}

impl<'cx> ArenaRefAlloc<'cx, str> for typed_arena::Arena<u8> {
    fn alloc(&'cx self, value: &str) -> &'cx str {
        self.alloc_str(value)
    }
}

impl<'cx, T> ArenaAlloc<'cx, T> for typed_arena::Arena<T> {
    fn alloc(&'cx self, value: T) -> &'cx T {
        self.alloc(value)
    }
}

mod sealed {
    /// A mapping from an arena allocator type to its allocated value type.
    pub trait ArenaAllocator<'cx> {
        type Allocated;
    }
}

pub type ArenaAllocator<T> = typed_arena::Arena<T>;
impl<'cx, T: 'cx> sealed::ArenaAllocator<'cx> for ArenaAllocator<T> {
    type Allocated = &'cx T;
}

pub type ArenaAllocated<'cx, Allocator> = <Allocator as sealed::ArenaAllocator<'cx>>::Allocated;
