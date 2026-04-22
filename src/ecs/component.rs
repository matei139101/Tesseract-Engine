use std::collections::HashMap;
use super::Entity;

pub struct ComponentStorage<T> {
    data: HashMap<Entity, T>
}
