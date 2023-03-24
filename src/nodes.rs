use std::{
    any::type_name,
    collections::HashMap,
    fmt::Debug,
    marker::PhantomData,
    num::NonZeroUsize,
    ops::{Index, IndexMut},
    sync::atomic::AtomicUsize,
};

pub struct NodeID<T>(NonZeroUsize, PhantomData<T>);

impl<T> NodeID<T> {
    pub fn new() -> Self {
        static CURRENT_ID: AtomicUsize = AtomicUsize::new(1);
        let id = CURRENT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self(id.try_into().unwrap(), PhantomData)
    }
}

impl<T> Default for NodeID<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Debug for NodeID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!("NodeID<{}>", type_name::<T>()))
            .field(&self.0)
            .finish()
    }
}

impl<T> Clone for NodeID<T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}

impl<T> Copy for NodeID<T> {}

impl<T> PartialEq for NodeID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for NodeID<T> {}

impl<T> PartialOrd for NodeID<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for NodeID<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> std::hash::Hash for NodeID<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

pub struct Nodes<T> {
    nodes: HashMap<NodeID<T>, T>,
}

impl<T> Nodes<T> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, node: T) -> NodeID<T> {
        let id = NodeID::new();
        let previous = self.nodes.insert(id, node);
        debug_assert!(previous.is_none());
        id
    }

    pub fn get(&self, id: NodeID<T>) -> Option<&T> {
        self.nodes.get(&id)
    }

    pub fn get_mut(&mut self, id: NodeID<T>) -> Option<&mut T> {
        self.nodes.get_mut(&id)
    }
}

impl<T> Default for Nodes<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<NodeID<T>> for Nodes<T> {
    type Output = T;

    fn index(&self, index: NodeID<T>) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<NodeID<T>> for Nodes<T> {
    fn index_mut(&mut self, index: NodeID<T>) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
