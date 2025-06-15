use std::{
    cell::RefCell,
    hash::{Hash, Hasher},
};

use rustc_hash::FxHashSet;

mod sealed {
    /// A zero-sized type used to seal the [`Interned`] struct.
    /// This prevents external implementation on it.
    ///
    /// [`Interned`]: super::Interned
    pub struct SealedZst<T>(pub std::marker::PhantomData<T>);
    /// `Clone` and `Copy` implementations for the type, which cannot be `derive`d
    /// they do not know `T` is going to be bound to `PhantomData`.
    impl<T> Clone for SealedZst<T> {
        fn clone(&self) -> Self {
            *self
        }
    }
    impl<T> Copy for SealedZst<T> {}

    /// A mapping from an interner type to its interned value type.
    /// Used to define the [`Interned`] type alias.
    ///
    /// This trait is sealed to prevent external implementations.
    ///
    /// [`Interned`]: super::Interned
    pub trait InternerTrait {
        type Interned;
    }
}

/// A string interner that uses an arena for storage.
pub type StrInterner<'cx> = FxHashInterner<'cx, str, typed_arena::Arena<u8>>;

/// An interned string type that uses the [`StrInterner`] for interning.
pub type Str<'cx> = Interned<StrInterner<'cx>>;

/// A default, generic interner that uses an arena for storage.
pub type Interner<'cx, T> = FxHashInterner<'cx, T, typed_arena::Arena<T>>;

/// A generic interner that does not allocate new values.
pub type RefOnlyInterner<'cx, T> = FxHashInterner<'cx, T, NoAlloc>;

/// A marker type indicating that no allocation is performed.
pub struct NoAlloc;

/// A type alias for the interned value type.
pub type Interned<Interner> = <Interner as sealed::InternerTrait>::Interned;

/// A unique `impl` of [`sealed::InternerTrait`].
impl<'cx, T: ?Sized, Arena> sealed::InternerTrait for FxHashInterner<'cx, T, Arena> {
    type Interned = InternedRef<'cx, T, Self>;
}

/// A unique reference to an interned value.
///
/// This type is used to represent a value that has been interned in an interner.
pub struct InternedRef<'a, T: ?Sized, Interner>(pub &'a T, sealed::SealedZst<Interner>);

impl<'a, T: ?Sized, Interner> std::ops::Deref for InternedRef<'a, T, Interner> {
    type Target = &'a T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: ?Sized, Interner> InternedRef<'a, T, Interner> {
    /// Creates a new interned handle from a reference to the value.
    ///
    /// This function should not be `pub`lic to prevent from breaking the invariant
    /// for `Eq`, `PartialEq`, and `Hash` traits.
    fn new(value: &'a T) -> Self {
        Self(value, sealed::SealedZst(std::marker::PhantomData))
    }
}

impl<T: ?Sized, Interner> Clone for InternedRef<'_, T, Interner> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: ?Sized, Interner> Copy for InternedRef<'_, T, Interner> {}

impl<T: std::fmt::Debug + ?Sized, Interner> std::fmt::Debug for InternedRef<'_, T, Interner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: ?Sized, Interner> PartialEq for InternedRef<'_, T, Interner> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}
impl<T: ?Sized, Interner> Eq for InternedRef<'_, T, Interner> {}

impl<T: ?Sized, Interner> Hash for InternedRef<'_, T, Interner> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.0, state);
    }
}

impl<T: ?Sized + PartialOrd, Interner> PartialOrd for InternedRef<'_, T, Interner> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other.0)
    }
}
impl<T: ?Sized + Ord, Interner> Ord for InternedRef<'_, T, Interner> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(other.0)
    }
}

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

/// A handler for copying references into a hash set without allocating a new one.
pub trait RefCopy<'cx, T: ?Sized> {
    /// Copies the value into the container, without allocating a new one.
    fn copy(&'cx self, value: &'cx T);
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

/// Any type can be [`RefCopy`] as it does not allocate. Especially [`typed_arena::Arena<T>`]
/// can be used to intern references without allocation.
impl<'cx, T: ?Sized, Any> RefCopy<'cx, T> for Any {
    fn copy(&'cx self, _value: &'cx T) {
        // No-op, as this trait is used to indicate that the value is copied
        // without allocating a new one.
    }
}

/// A hash-based interner that uses an arena for storage.
///
/// Use [`StrInterner`] or [`Interner`] for standard uses.
pub struct FxHashInterner<'cx, T: ?Sized, Arena> {
    arena: Arena,
    map: RefCell<FxHashSet<&'cx T>>,
}

impl<'cx, T: ?Sized, Arena: Default> Default for FxHashInterner<'cx, T, Arena> {
    /// Creates a new interning table.
    fn default() -> Self {
        Self {
            arena: Arena::default(),
            map: RefCell::new(FxHashSet::default()),
        }
    }
}

impl<'cx, T: Hash + Eq, Arena: ArenaAlloc<'cx, T>> FxHashInterner<'cx, T, Arena> {
    /// Interns the given path (if not already present), and returns a lightweight handle.
    ///
    /// This function will allocate the value in the arena if it is not already present.
    ///
    /// This function restricts `self` to be valid for the lifetime `'cx` for the arena,
    /// while `map` does not need to do so.
    ///
    /// Do not use multiple interners for the same interner type, as the returned [`Interned`] value
    /// cannot tell which interner generated it.
    pub fn intern(&'cx self, value: T) -> InternedRef<'cx, T, Self> {
        let mut lock = self.map.borrow_mut();
        if let Some(&existing) = lock.get(&value) {
            return InternedRef::new(existing);
        }

        let new_ref = self.arena.alloc(value);
        lock.insert(new_ref);
        InternedRef::new(new_ref)
    }
}

impl<'cx, T: ?Sized + Hash + Eq, Arena: ArenaRefAlloc<'cx, T>> FxHashInterner<'cx, T, Arena> {
    /// Reference version of [`FxHashInterner::intern`]. See its documentation for details.
    pub fn intern_ref<'any>(&'cx self, value: &'any T) -> InternedRef<'cx, T, Self> {
        let mut lock = self.map.borrow_mut();
        if let Some(&existing) = lock.get(value) {
            return InternedRef::new(existing);
        }

        let new_ref = self.arena.alloc(value);
        lock.insert(new_ref);
        InternedRef::new(new_ref)
    }
}

impl<'cx, T: ?Sized + Hash + Eq, Arena: RefCopy<'cx, T>> FxHashInterner<'cx, T, Arena> {
    /// Reference version of [`FxHashInterner::intern`].
    /// See its documentation for details.
    ///
    /// This function copies the value into the interner without allocating a new one,
    /// thus this function takes a reference to the value that lasts for the lifetime `'cx`.
    pub fn intern_ref_noalloc<'a>(&'cx self, value: &'a T) -> InternedRef<'cx, T, Self>
    where
        'a: 'cx,
    {
        let mut lock = self.map.borrow_mut();
        if let Some(&existing) = lock.get(value) {
            return InternedRef::new(existing);
        }

        self.arena.copy(value);
        lock.insert(value);
        InternedRef::new(value)
    }
}
