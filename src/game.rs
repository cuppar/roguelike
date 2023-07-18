pub mod input;
pub mod map;
pub mod object;
pub mod render;

use map::Map;

pub struct Game {
    pub map: Map,
}
