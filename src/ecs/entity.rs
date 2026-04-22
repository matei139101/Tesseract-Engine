/// An unique identifier for an object in the simulation.
///
///Entity IDs should never constructed directly - always obatain them through ['']
pub struct Entity {
    id: u32
}

/// Handles allocation, freeing and recycling of ['Entity'] IDs.
///
/// Hands out unique IDs to spawned entities and frees IDs when entities are despawned.
/// Additionally, reuses stale IDs when possible from despawned entities.
///
/// Should not be used directly and only used by [''].
#[derive(Default)]
pub struct EntityAllocator {
    next: u32,
    free: Vec<u32>,
}

impl EntityAllocator {
    /// Returns an ['Entity'] with a unique ID.
    pub fn alloc(&mut self) -> Entity {
        if let Some(id) = self.free.pop() {
            Entity {id}
        } else {
            let id = self.next;
            self.next += 1;
            Entity { id }
        }
    }

    /// Frees an ID to be reused for future new ['Entity'] creation.
    pub fn free(&mut self, entity: Entity) {
        self.free.push(entity.id);
    }
}
