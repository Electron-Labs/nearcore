use crate::routing::Edge;
use near_primitives::borsh::maybestd::borrow::Borrow;
use near_primitives::network::PeerId;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

/// Wraps around `Edge` struct. The main feature of this struct, is that it's hashed by
/// `(Edge::key.0, Edge::key.1)` pair instead of `(Edge::key.0, Edge::key.1, Edge::nonce)`
/// triple.
#[derive(Eq, PartialEq)]
pub struct EdgeIndexedByKey {
    inner: Edge,
}

impl Borrow<(PeerId, PeerId)> for EdgeIndexedByKey {
    fn borrow(&self) -> &(PeerId, PeerId) {
        self.inner.key()
    }
}

impl Hash for EdgeIndexedByKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.key().hash(state);
    }
}

pub(crate) struct EdgeSet {
    repr: HashSet<EdgeIndexedByKey>,
}

impl Default for EdgeSet {
    fn default() -> Self {
        Self { repr: HashSet::new() }
    }
}

impl EdgeSet {
    /// Note: We need to remove the value first.
    /// The insert inside HashSet, will not remove existing element if it has the same key.
    pub(crate) fn insert(&mut self, edge: Edge) -> bool {
        self.remove(edge.key());
        self.repr.insert(EdgeIndexedByKey { inner: edge })
    }

    pub(crate) fn get(&self, key: &(PeerId, PeerId)) -> Option<&Edge> {
        self.repr.get(key).map(|v| &v.inner)
    }

    pub(crate) fn remove(&mut self, key: &(PeerId, PeerId)) -> bool {
        self.repr.remove(key)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Edge> + '_ {
        self.repr.iter().map(|it| &it.inner)
    }

    #[allow(unused)]
    pub(crate) fn len(&self) -> usize {
        self.repr.len()
    }
}
