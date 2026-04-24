use super::Entity;
use std::collections::HashMap;

/// Used for storing component data into [`World`]
///
/// Stores a map of data of any type T accssible by [`Entity`]
///
/// Elements should never be manually added or removed and only accessed through provided methods.
pub struct ComponentStorage<T> {
    data: HashMap<Entity, T>,
}

impl<T> ComponentStorage<T> {
    /// Returns an empty [`ComponentStorage`]
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Inserts an element with the provided [`Entity`] as key and value
    pub fn insert(&mut self, entity: Entity, value: T) {
        self.data.insert(entity, value);
    }

    /// Returns an [`Option`] containing a reference to the value indexed by a provided [`Entity`]
    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.data.get(&entity)
    }

    /// Returns an [`Option`] containing a mutable reference to the value indexed by a provided [`Entity`]
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.data.get_mut(&entity)
    }

    /// Removes a value indexed by a provided [`Entity`]
    pub fn remove(&mut self, entity: Entity) {
        self.data.remove(&entity);
    }

    /// Returns an iterator over all stored components and their associated [`Entity`]
    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.data.iter().map(|(e, c)| (*e, c))
    }
}
