/*
 * Example Component: Position
 * Component are two parts, the component itself and the store
 * Component: Simply a data structure
 * Store: A Vec of the Component; Must implement Storage and Default
*/
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Debug, PartialEq)]
// Velocity
pub struct Velocity {
    pub vel: f32,
}
