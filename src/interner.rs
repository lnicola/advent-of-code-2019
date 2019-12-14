use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::hash::{BuildHasher, Hash};
use std::ops::Index;

pub struct Interner<T, S = RandomState>(HashMap<T, usize, S>);

impl<T: Hash + Eq> Interner<T, RandomState> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl<T: Hash + Eq, S: BuildHasher> Interner<T, S> {
    pub fn with_hasher(hash_builder: S) -> Self {
        Self(HashMap::with_hasher(hash_builder))
    }
}

impl<T: Hash + Eq, S: BuildHasher> Interner<T, S> {
    pub fn insert(&mut self, value: T) -> usize {
        let len = self.0.len();
        *self.0.entry(value).or_insert(len)
    }

    pub fn get<Q: ?Sized>(&self, value: &Q) -> Option<&usize>
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get(value)
    }
}

impl<T, Q: ?Sized, S> Index<&Q> for Interner<T, S>
where
    T: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash,
    S: BuildHasher,
{
    type Output = usize;

    #[inline]
    fn index(&self, key: &Q) -> &usize {
        self.get(key).expect("no entry found for key")
    }
}

impl<T, S> Debug for Interner<T, S>
where
    T: Eq + Hash + Debug,
    S: BuildHasher,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
