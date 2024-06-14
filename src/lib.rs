#![doc = include_str!("../README.md")]

#[cfg(test)]
mod tests;
mod borrowed;

pub use borrowed::Borrowed;

use core::{
    borrow::{Borrow, BorrowMut},
    cell::Cell,
    cmp::Ordering,
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicU64, Ordering as MOrd},
};
use std::{
    rc::Rc,
    sync::Arc,
    collections::hash_map::DefaultHasher,
};

/// storage trait for storing hash status
pub trait HashStorer {
    /// Clear stored hash code to none
    fn clear(&mut self);

    /// Get stored hash code
    fn get(&self) -> Option<u64>;

    /// if stored hash code is uninit, call init func
    ///
    /// return inited hash code
    fn get_or_init<F>(&self, f: F) -> u64
    where F: FnOnce() -> u64;

    fn hash_one<T, H>(value: &T) -> u64
    where T: ?Sized + Hash,
          H: Hasher + Default,
          Self: Default,
    {
        Self::default()
            .get_or_init(|| {
                let mut hasher = H::default();
                value.hash(&mut hasher);
                hasher.finish()
            })
    }
}

impl HashStorer for Cell<u64> {
    fn clear(&mut self) {
        self.set(0)
    }

    fn get(&self) -> Option<u64> {
        let n = self.get();
        if n == 0 { return None; }
        Some(n)
    }

    fn get_or_init<F>(&self, f: F) -> u64
    where F: FnOnce() -> u64,
    {
        HashStorer::get(self)
            .unwrap_or_else(|| {
                let mut n = f();
                if n == 0 { n = u64::MAX >> 2 }
                self.set(n);
                n
            })
    }
}
impl HashStorer for AtomicU64 {
    fn clear(&mut self) {
        self.store(0, MOrd::Relaxed)
    }

    fn get(&self) -> Option<u64> {
        let n = self.load(MOrd::Relaxed);
        if n == 0 { return None; }
        Some(n)
    }

    fn get_or_init<F>(&self, f: F) -> u64
    where F: FnOnce() -> u64,
    {
        HashStorer::get(self)
            .unwrap_or_else(|| {
                let mut n = f();
                if n == 0 { n = u64::MAX >> 2 }
                self.store(n, MOrd::Relaxed);
                n
            })
    }
}
impl<T: HashStorer + Default> HashStorer for Rc<T> {
    fn get(&self) -> Option<u64> {
        <T as HashStorer>::get(&**self)
    }

    fn clear(&mut self) {
        Rc::get_mut(self)
            .map(T::clear)
            .unwrap_or_else(|| {
                *self = Default::default()
            })
    }

    fn get_or_init<F>(&self, f: F) -> u64
    where F: FnOnce() -> u64,
    {
        <T as HashStorer>::get_or_init(&**self, f)
    }

    fn hash_one<T1, H>(value: &T1) -> u64
    where T1: ?Sized + Hash,
          H: Hasher + Default,
          Self: Default,
    {
        T::hash_one::<T1, H>(value)
    }
}
impl<T: HashStorer + Default> HashStorer for Arc<T> {
    fn get(&self) -> Option<u64> {
        <T as HashStorer>::get(&**self)
    }

    fn clear(&mut self) {
        Arc::get_mut(self)
            .map(T::clear)
            .unwrap_or_else(|| {
                *self = Default::default()
            })
    }

    fn get_or_init<F>(&self, f: F) -> u64
    where F: FnOnce() -> u64,
    {
        <T as HashStorer>::get_or_init(&**self, f)
    }

    fn hash_one<T1, H>(value: &T1) -> u64
    where T1: ?Sized + Hash,
          H: Hasher + Default,
          Self: Default,
    {
        T::hash_one::<T1, H>(value)
    }
}

/// A wrapper for storing hash results to avoid running costly hash functions
/// multiple times without modifying the value
///
/// # Examples
/// ```
/// # use hash_on_write::How;
/// # use std::collections::HashSet;
/// let mut x = How::new_default("foo".to_owned());
///
/// assert!(! How::is_hashed(&x));
/// HashSet::new().insert(&x);
/// assert!(How::is_hashed(&x));
///
/// How::make_mut(&mut x).push('!');
/// assert!(! How::is_hashed(&x));
/// assert_eq!(*x, "foo!");
/// ```
///
/// ---
/// Due to the inability of the stored hashcode to replicate the action of `T::hash,`
/// it is not possible to implement [`Borrow<T>`]
///
/// [`Borrow<T>`]: core::borrow::Borrow
pub struct How<T: ?Sized, H = DefaultHasher, S = Cell<u64>> {
    _hasher: PhantomData<H>,
    hashcode: S,
    value: T,
}
impl<T, H, S> Default for How<T, H, S>
where T: Default,
      S: Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}
impl<T: ?Sized, H, S> AsRef<T> for How<T, H, S> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}
impl<T: ?Sized, H, S: HashStorer> AsMut<T> for How<T, H, S> {
    fn as_mut(&mut self) -> &mut T {
        How::make_mut(self)
    }
}
impl<T: ?Sized, H, S> AsRef<Self> for How<T, H, S> {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl<T: ?Sized, H, S> AsMut<Self> for How<T, H, S> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
impl<T, Q, H, S> Borrow<Borrowed<Q, H, S>> for How<T, H, S>
where T: ?Sized + Borrow<Q>,
      Q: ?Sized,
{
    fn borrow(&self) -> &Borrowed<Q, H, S> {
        Borrowed::make_ref(self.value.borrow())
    }
}
impl<T, Q, H, S> BorrowMut<Borrowed<Q, H, S>> for How<T, H, S>
where T: ?Sized + BorrowMut<Q>,
      Q: ?Sized,
{
    fn borrow_mut(&mut self) -> &mut Borrowed<Q, H, S> {
        Borrowed::make_mut(self.value.borrow_mut())
    }
}
impl<T: ?Sized + Debug, H, S: Debug> Debug for How<T, H, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("How")
            .field("hashcode", &self.hashcode)
            .field("value", &&self.value)
            .finish()
    }
}
impl<T: ?Sized + Clone, H, S: Clone> Clone for How<T, H, S> {
    fn clone(&self) -> Self {
        Self {
            _hasher: PhantomData,
            hashcode: self.hashcode.clone(),
            value: self.value.clone(),
        }
    }
}
impl<T: ?Sized + Eq, H, S: HashStorer> Eq for How<T, H, S> { }
impl<T: ?Sized + Ord, H, S: HashStorer> Ord for How<T, H, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
impl<T: ?Sized + PartialEq, H, S: HashStorer> PartialEq for How<T, H, S> {
    fn eq(&self, other: &Self) -> bool {
        self.hashcode.get()
            .zip(other.hashcode.get())
            .map_or(true, |(a, b)| a == b)
            && self.value == other.value
    }
}
impl<T: ?Sized + PartialEq, H, S> PartialEq<T> for How<T, H, S> {
    fn eq(&self, other: &T) -> bool {
        **self == *other
    }
}
impl<T: ?Sized + PartialOrd, H, S: HashStorer> PartialOrd for How<T, H, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl<T: ?Sized + PartialOrd, H, S> PartialOrd<T> for How<T, H, S> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        (**self).partial_cmp(other)
    }
}
impl<T: ?Sized, H, S> Deref for How<T, H, S> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T, H, S> DerefMut for How<T, H, S>
where T: ?Sized + Hash,
      H: Hasher + Default,
      S: HashStorer,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        Self::make_mut(self)
    }
}
impl<T, IH, S> Hash for How<T, IH, S>
where T: ?Sized + Hash,
      IH: Hasher + Default,
      S: HashStorer,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        Self::make_hash(self)
            .hash(state)
    }
}
impl<T, H, S> From<T> for How<T, H, S>
where H: Hasher + Default,
      S: HashStorer + Default,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
impl<T> How<T> {
    /// new, but use [`DefaultHasher`]
    ///
    /// [`DefaultHasher`]: std::collections::hash_map::DefaultHasher
    pub fn new_default(value: T) -> Self {
        How {
            _hasher: PhantomData,
            hashcode: Default::default(),
            value,
        }
    }
}
impl<T, H, S: Default> How<T, H, S> {
    /// New a wrapped value
    pub fn new(value: T) -> Self {
        How {
            _hasher: PhantomData,
            hashcode: Default::default(),
            value,
        }
    }
}

impl<T, H, S> How<T, H, S> {
    /// Consume `self` into wrapped value
    pub fn into_inner(this: Self) -> T {
        this.value
    }
}
impl<T: ?Sized, H, S: HashStorer> How<T, H, S> {
    /// Get mutable and clear hash cache
    pub fn make_mut(this: &mut Self) -> &mut T {
        this.hashcode.clear();
        &mut this.value
    }

    /// Get hash cache status
    pub fn hash_code(this: &Self) -> Option<u64> {
        this.hashcode.get()
    }

    /// Get hash cache status is cached,
    /// like `How::hash_code(&value).is_some()`
    pub fn is_hashed(this: &Self) -> bool {
        Self::hash_code(this).is_some()
    }
}
impl<T, H, S> How<T, H, S>
where T: ?Sized + Hash,
      H: Default + Hasher,
      S: HashStorer,
{
    /// Get or init hash cache
    pub fn make_hash(this: &Self) -> u64 {
        this.hashcode.get_or_init(|| {
            let mut inner_hasher = H::default();
            this.value.hash(&mut inner_hasher);
            inner_hasher.finish()
        })
    }
}
