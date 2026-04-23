use std::collections::HashMap;
use super::Entity;

pub struct ComponentStorage<T> {
    data: HashMap<Entity, T>
}

impl <T> ComponentStorage<T> {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn insert(&mut self, entity: Entity, value: T) {
        self.data.insert(entity, value);
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.data.get(&entity)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.data.get_mut(&entity)
    }

    pub fn remove(&mut self, entity: Entity) {
        self.data.remove(&entity);
    }

    pub fn iter(&self) ->  {
        self.data.iter()
    }
}
