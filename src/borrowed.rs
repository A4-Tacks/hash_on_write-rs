use core::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
    ops::{Deref, DerefMut},
};

use crate::HashStorer;

/// A hash wrapper, hash with behavior like [`How`]
///
/// # Examples
/// ```
/// # use hash_on_write::{How, Borrowed};
/// # use std::collections::HashSet;
/// let mut set: HashSet<How<String>> = HashSet::new();
///
/// assert!(! set.contains(Borrowed::make_ref("a")));
/// set.insert("a".to_owned().into());
/// assert!(set.contains(Borrowed::make_ref("a")));
/// ```
///
/// [`How`]: crate::How
#[repr(transparent)]
pub struct Borrowed<T: ?Sized, H, S> {
    _hasher: PhantomData<H>,
    _state: PhantomData<S>,
    pub value: T,
}
impl<T: ?Sized, H, S> AsRef<Self> for Borrowed<T, H, S> {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl<T: ?Sized, H, S> AsMut<Self> for Borrowed<T, H, S> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
impl<T: ?Sized, H, S> AsRef<T> for Borrowed<T, H, S> {
    fn as_ref(&self) -> &T {
        self
    }
}
impl<T: ?Sized, H, S> AsMut<T> for Borrowed<T, H, S> {
    fn as_mut(&mut self) -> &mut T {
        self
    }
}
impl<T, H, S> Borrowed<T, H, S> {
    /// Create a [`Borrowed`]
    ///
    /// [`Borrowed`]: crate::Borrowed
    pub fn new(value: T) -> Self {
        Self {
            _hasher: PhantomData,
            _state: PhantomData,
            value,
        }
    }
}
impl<T: ?Sized, H, S> Borrowed<T, H, S> {
    /// transmute reference to [`Borrowed`] reference
    ///
    /// [`Borrowed`]: crate::Borrowed
    pub fn make_ref(value: &T) -> &Self {
        unsafe { transmute(value) }
    }

    /// transmute mutable reference to [`Borrowed`] mutable reference
    ///
    /// [`Borrowed`]: crate::Borrowed
    pub fn make_mut(value: &mut T) -> &mut Self {
        unsafe { transmute(value) }
    }
}
impl<T: ?Sized + Debug, H, S> Debug for Borrowed<T, H, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Borrowed")
            .field(&&self.value)
            .finish()
    }
}
impl<T: ?Sized + Copy, H, S> Copy for Borrowed<T, H, S> { }
impl<T: Clone, H, S> Clone for Borrowed<T, H, S> {
    fn clone(&self) -> Self {
        Self::new(self.value.clone())
    }
}
impl<T: ?Sized + PartialEq, H, S> PartialEq for Borrowed<T, H, S> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}
impl<T: ?Sized + PartialOrd, H, S> PartialOrd for Borrowed<T, H, S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl<T: ?Sized + Ord, H, S> Ord for Borrowed<T, H, S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
impl<T: ?Sized + Eq, H, S> Eq for Borrowed<T, H, S> { }
impl<T, H, S> Hash for Borrowed<T, H, S>
where T: ?Sized + Hash,
      H: Hasher + Default,
      S: HashStorer + Default,
{
    fn hash<H1: Hasher>(&self, state: &mut H1) {
        S::hash_one::<_, H>(&self.value)
            .hash(state)
    }
}
impl<T: ?Sized, H, S> DerefMut for Borrowed<T, H, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
impl<T: ?Sized, H, S> Deref for Borrowed<T, H, S> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
