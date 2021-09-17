use crate::{Storage};

/*
 * Example Component: Position
 * Component are two parts, the component itself and the store
 * Component: Simply a data structure
 * Store: A Vec of the Component; Must implement Storage and Default
*/
pub struct Position {
    pub x: i64,
    pub y: i64,
}

// Velocity
pub struct Velocity {
    pub vel: f32,
}
