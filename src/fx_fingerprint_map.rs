use std::{
    collections::HashMap,
    hash::{BuildHasherDefault, Hash, Hasher},
    iter::FromIterator,
    marker::PhantomData,
};

use rustc_hash::FxHasher;

/// A map that stores only the 64-bit FxHasher fingerprint of each key, not the
/// keys themselves.
///
/// WARNING: Key collisions are extremely rare, but if two distinct keys produce
/// the same fingerprint, their values will overwrite each other with no way to
/// distinguish the original keys.
#[derive(Clone, Debug)]
pub struct FxFingerprintMap<K, V> {
    inner: HashMap<
        u64,
        V,
        // Fingerprints are already u64, so no second layer of hashing needed.
        NoHash,
    >,
    // No actual usage of type K (since keys are not stored), so use phantom
    // data to maintain type safety.
    _marker: PhantomData<K>,
}

// ────────────────────────────────────────────────────────────
// Public API.
// ────────────────────────────────────────────────────────────

impl<K: Hash, V> FxFingerprintMap<K, V> {
    /// Creates an empty map.
    pub fn new() -> Self {
        Self {
            inner: HashMap::default(),
            _marker: PhantomData,
        }
    }

    /// Creates an empty map with enough capacity for `cap` elements.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: HashMap::with_capacity_and_hasher(
                cap,
                BuildHasherDefault::<IdentityHasher>::default(),
            ),
            _marker: PhantomData,
        }
    }

    /// Returns the number of elements in the map.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// True if the map contains *no* elements.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns a reference to the value for `key`, or `None` if absent.
    #[inline(always)]
    pub fn get<Q: ?Sized + Hash>(&self, key: &Q) -> Option<&V> {
        self.inner.get(&fingerprint(key))
    }

    /// Mutable variant of [`get`](Self::get).
    #[inline(always)]
    pub fn get_mut<Q: ?Sized + Hash>(&mut self, key: &Q) -> Option<&mut V> {
        self.inner.get_mut(&fingerprint(key))
    }

    /// Adds or replaces the value for `key`.
    ///
    /// Returns the old value if there was one.
    #[inline(always)]
    pub fn insert(&mut self, key: &K, value: V) -> Option<V> {
        self.inner.insert(fingerprint(key), value)
    }

    /// True if a value has already been stored for `key`.
    #[inline(always)]
    pub fn contains_key<Q: ?Sized + Hash>(&self, key: &Q) -> bool {
        self.inner.contains_key(&fingerprint(key))
    }

    /// Removes and returns the value for `key`, if it exists.
    #[inline(always)]
    pub fn remove<Q: ?Sized + Hash>(&mut self, key: &Q) -> Option<V> {
        self.inner.remove(&fingerprint(key))
    }

    /// Clears the map, keeping the allocated buckets.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Returns an iterator visiting all values immutably.
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.inner.values()
    }

    /// Returns an iterator visiting all values mutably.
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.inner.values_mut()
    }

    /// Consumes the map and returns the underlying `HashMap`.
    pub fn into_inner(self) -> HashMap<u64, V, NoHash> {
        self.inner
    }

    /// Returns an iterator visiting all fingerprints and values immutably.
    pub fn iter_fps(&self) -> impl Iterator<Item = (&u64, &V)> {
        self.inner.iter()
    }

    /// Returns an iterator visiting all fingerprints and values mutably.
    pub fn iter_fps_mut(&mut self) -> impl Iterator<Item = (&u64, &mut V)> {
        self.inner.iter_mut()
    }

    /// Computes the 64-bit *FxHasher* fingerprint of `key`.
    #[inline(always)]
    pub fn fingerprint<Q: ?Sized + Hash>(key: &Q) -> u64 {
        fingerprint(key)
    }

    /// Builds a map directly from pre-computed **`(fingerprint, value)`**
    /// pairs.
    ///
    /// Fingerprints can be precomputed with the [`fingerprint`] method.
    ///
    /// Use this when you have already hashed the keys (or had to *consume* them
    /// to build each value) and therefore cannot pass the original keys to
    /// `collect()`.
    pub fn from_raw(iter: Vec<(u64, V)>) -> Self {
        let mut map = Self::new();
        map.inner.extend(iter);
        map
    }

    /// Transforms the values in the map using the given function.
    pub fn map<F, U>(&self, mut f: F) -> FxFingerprintMap<K, U>
    where
        F: FnMut(&V) -> U,
    {
        let inner = self.inner.iter().map(|(&fp, v)| (fp, f(v))).collect();
        FxFingerprintMap {
            inner,
            _marker: PhantomData,
        }
    }
}

// ────────────────────────────────────────────────────────────
// Trait impls.
// ────────────────────────────────────────────────────────────

impl<K: Hash, V> Default for FxFingerprintMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Hash, V> Extend<(K, V)> for FxFingerprintMap<K, V> {
    /// Extends the map from an iterator of `(key, value)` pairs.
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for (k, v) in iter {
            self.insert(&k, v);
        }
    }
}

impl<K: Hash, V> FromIterator<(K, V)> for FxFingerprintMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = FxFingerprintMap::new();
        map.extend(iter);
        map
    }
}

// ────────────────────────────────────────────────────────────
// Identity hasher.
// ────────────────────────────────────────────────────────────

/// Pass-through hasher that uses the input u64 directly as the lookup hash.
#[derive(Default)]
pub struct IdentityHasher(u64);

impl Hasher for IdentityHasher {
    fn write(&mut self, bytes: &[u8]) {
        debug_assert_eq!(bytes.len(), 8);
        self.0 = u64::from_ne_bytes(bytes.try_into().unwrap());
    }
    fn finish(&self) -> u64 {
        self.0
    }
}

pub type NoHash = BuildHasherDefault<IdentityHasher>;

// ────────────────────────────────────────────────────────────
// Helpers.
// ────────────────────────────────────────────────────────────

#[inline(always)]
fn fingerprint<H: ?Sized + Hash>(value: &H) -> u64 {
    let mut hasher = FxHasher::default();
    value.hash(&mut hasher);
    hasher.finish()
}
