// 3party
use rand::Rng;
use tcod::colors::*;
use tcod::map::FovAlgorithm;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

// own modules
use super::{object::*, render::PANEL_HEIGHT};

pub const MAX_ROOM_MONSTERS: i32 = 3;

pub const MAP_WIDTH: i32 = SCREEN_WIDTH;
pub const MAP_HEIGHT: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

pub type Map = Vec<Vec<Tile>>;

pub const ROOM_MAX_SIZE: i32 = 12;
pub const ROOM_MIN_SIZE: i32 = 8;
pub const MAX_ROOMS: i32 = 100;

pub const MAX_ROOM_ITEMS: i32 = 2;

pub const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
pub const FOV_LIGHT_WALLS: bool = true;
pub const TORCH_RADIUS: i32 = 10;
pub const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
pub const COLOR_LIGHT_WALL: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};
pub const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};
pub const COLOR_LIGHT_GROUND: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub explored: bool,
    pub block_sight: bool,
}
impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            explored: false,
            block_sight: false,
        }
    }
    pub fn wall() -> Self {
        Tile {
            blocked: true,
            explored: false,
            block_sight: true,
        }
    }
}

pub fn make_map(objects: &mut Vec<Object>) -> Map {
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let mut rooms = vec![];
    for _ in 0..MAX_ROOMS {
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE..=ROOM_MAX_SIZE);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE..=ROOM_MAX_SIZE);
        let x = rand::thread_rng().gen_range(0..(MAP_WIDTH - w));
        let y = rand::thread_rng().gen_range(0..(MAP_HEIGHT - h));

        let new_room = Rect::new(x, y, w, h);
        let failed = rooms
            .iter()
            .any(|other_room| new_room.intersects_with(other_room));
        if !failed {
            create_room(new_room, &mut map);
            let (new_x, new_y) = new_room.center();

            if rooms.is_empty() {
                objects[PLAYER].set_pos(new_x, new_y);
            } else {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rand::random() {
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }
                place_objects(new_room, &map, objects);
            }
            rooms.push(new_room);
        }
    }

    map
}

pub fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

pub fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    use std::cmp::max;
    use std::cmp::min;

    for x in min(x1, x2)..=max(x1, x2) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

pub fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    use std::cmp::max;
    use std::cmp::min;

    for y in min(y1, y2)..=max(y1, y2) {
        map[x as usize][y as usize] = Tile::empty();
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}
impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }
    pub fn center(&self) -> (i32, i32) {
        let c_x = (self.x1 + self.x2) / 2;
        let c_y = (self.y1 + self.y2) / 2;
        (c_x, c_y)
    }
    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}
pub fn place_objects(room: Rect, map: &Map, objects: &mut Vec<Object>) {
    let num_monsters = rand::thread_rng().gen_range(0..=MAX_ROOM_MONSTERS);
    for _ in 0..num_monsters {
        let x = rand::thread_rng().gen_range((room.x1 + 1)..room.x2);
        let y = rand::thread_rng().gen_range((room.y1 + 1)..room.y2);

        if !is_blocked(x, y, map, objects) {
            let mut monster = if rand::random::<f32>() < 0.8 {
                let mut orc = Object::new(x, y, 'o', "orc", DESATURATED_GREEN, true);
                orc.fighter = Some(Fighter {
                    max_hp: 10,
                    hp: 10,
                    defense: 0,
                    power: 3,
                    on_death: DeathCallback::Monster,
                });
                orc.ai = Some(Ai::Basic);
                orc
            } else {
                let mut troll = Object::new(x, y, 'T', "Troll", DARKER_GREEN, true);
                troll.fighter = Some(Fighter {
                    max_hp: 16,
                    hp: 16,
                    defense: 1,
                    power: 4,
                    on_death: DeathCallback::Monster,
                });
                troll.ai = Some(Ai::Basic);
                troll
            };

            monster.alive = true;
            objects.push(monster);
        }
    }
    let num_items = rand::thread_rng().gen_range(0..=MAX_ROOM_ITEMS);
    for _ in 0..num_items {
        let x = rand::thread_rng().gen_range((room.x1 + 1)..room.x2);
        let y = rand::thread_rng().gen_range((room.y1 + 1)..room.y2);

        if !is_blocked(x, y, map, objects) {
            let dice = rand::random::<f32>();
            let item = if dice < 0.7 {
                let mut object = Object::new(x, y, '!', "healing potion", VIOLET, false);
                object.item = Some(Item::Heal);
                object
            } else if dice < 0.7 + 0.1 {
                let mut object =
                    Object::new(x, y, '#', "scroll of lightning bolt", LIGHT_YELLOW, false);
                object.item = Some(Item::Lightning);
                object
            } else {
                let mut object =
                    Object::new(x, y, '#', "scroll of confusion", LIGHT_YELLOW, false);
                object.item = Some(Item::Confuse);
                object
            };
            objects.push(item);
        }
    }
}

pub fn is_blocked(x: i32, y: i32, map: &Map, objects: &[Object]) -> bool {
    if map[x as usize][y as usize].blocked {
        return true;
    }
    objects.iter().any(|o| o.blocks && o.pos() == (x, y))
}
