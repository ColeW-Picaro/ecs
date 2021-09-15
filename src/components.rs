use crate::{Storage};

/*
 * Example Component: Position
 * Component are two parts, the component itself and the store
 * Component: Simply a data structure
 * Store: A Vec of the Component; Must implement Storage and Default
*/
pub struct Position {
    x: i64,
    y: i64,
}

pub struct PositionStore {
    storage: Vec<Option<Position>>
}

impl Storage for PositionStore {
    fn allocate(&mut self) {
        self.storage.push(None);
    }
}

impl Default for PositionStore {
    fn default() -> PositionStore {
        PositionStore { storage: Vec::new() }
    }
}

// Velocity
pub struct Velocity {
    vel: f32,
}

pub struct VelocityStore {
    storage: Vec<Option<Velocity>>
}

impl Storage for VelocityStore {
    fn allocate(&mut self) {
        self.storage.push(None);
    }
}

impl Default for VelocityStore {
    fn default() -> VelocityStore {
        VelocityStore { storage: Vec::new() }
    }
}
