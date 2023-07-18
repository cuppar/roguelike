// 3party
use tcod::colors::*;
use tcod::console::*;

// own module
use super::map::*;
use super::*;

pub const PLAYER: usize = 0;

pub fn player_move_or_attack(dx: i32, dy: i32, game: &Game, objects: &mut [Object]) {
    let x = objects[PLAYER].x + dx;
    let y = objects[PLAYER].y + dy;

    let target_id = objects.iter().position(|o| o.pos() == (x, y));
    match target_id {
        Some(target_id) => {
            println!(
                "The {} laughs at your puny efforts to attack him!",
                objects[target_id].name
            );
        }
        None => {
            Object::move_by(PLAYER, dx, dy, &game.map, objects);
        }
    }
}

#[derive(Debug)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    pub char: char,
    pub color: Color,
    pub name: String,
    pub blocks: bool,
    pub alive: bool,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, name: &str, color: Color, blocks: bool) -> Self {
        Object {
            x,
            y,
            char,
            color,
            blocks,
            name: name.into(),
            alive: false,
        }
    }

    pub fn move_by(index: usize, dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
        let new_x = objects[index].x + dx;
        let new_y = objects[index].y + dy;
        if new_x >= MAP_WIDTH || new_x < 0 || new_y >= MAP_HEIGHT || new_y < 0 {
            return;
        }

        if !is_blocked(new_x, new_y, map, objects) {
            objects[index].set_pos(new_x, new_y);
        }
    }

    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}
