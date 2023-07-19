// 3party
use tcod::colors::*;
use tcod::console::*;

// own module
use super::map::*;
use super::render::*;
use super::*;

pub const PLAYER: usize = 0;

pub fn player_move_or_attack(dx: i32, dy: i32, game: &Game, objects: &mut [Object]) {
    let x = objects[PLAYER].x + dx;
    let y = objects[PLAYER].y + dy;

    let target_id = objects
        .iter()
        .position(|o| o.pos() == (x, y) && o.fighter.is_some());
    match target_id {
        Some(target_id) => {
            let (player, target) = mut_two(PLAYER, target_id, objects);
            player.attack(target);
        }
        None => {
            Object::move_by(PLAYER, dx, dy, &game.map, objects);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
    pub on_death: DeathCallback,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeathCallback {
    Player,
    Monster,
}
impl DeathCallback {
    fn callback(self, object: &mut Object) {
        use DeathCallback::*;
        let callback: fn(&mut Object) = match self {
            Monster => monster_death,
            Player => player_death,
        };
        callback(object);
    }
}

fn monster_death(monster: &mut Object) {
    println!("{} is dead!", monster.name);
    monster.char = '%';
    monster.color = DARK_RED;
    monster.blocks = false;
    monster.fighter = None;
    monster.ai = None;
    monster.name = format!("remains of {}", monster.name);
}
fn player_death(player: &mut Object) {
    println!("You die!");
    player.char = '%';
    player.color = DARK_RED;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ai {
    Basic,
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
    pub fighter: Option<Fighter>,
    pub ai: Option<Ai>,
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
            fighter: None,
            ai: None,
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
    pub fn distance_to(&self, other: &Object) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }
    pub fn take_damage(&mut self, damage: i32) {
        if let Some(fighter) = self.fighter.as_mut() {
            if damage > 0 {
                fighter.hp -= damage;
            }
        }
        if let Some(fighter) = self.fighter {
            if fighter.hp <= 0 {
                self.alive = false;
                fighter.on_death.callback(self);
            }
        }
    }
    pub fn attack(&mut self, target: &mut Object) {
        let damage = self.fighter.map_or(0, |f| f.power) - target.fighter.map_or(0, |f| f.defense);
        if damage > 0 {
            println!(
                "{} attack {} for {} hit points.",
                self.name, target.name, damage
            );
            target.take_damage(damage);
        } else {
            println!("{} attack {} but it has no effect!", self.name, target.name);
        }
    }
}

pub fn move_towards(id: usize, target_x: i32, target_y: i32, map: &Map, objects: &mut [Object]) {
    let dx = target_x - objects[id].x;
    let dy = target_y - objects[id].y;
    let dis = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();
    let dx = (dx as f32 / dis).round() as i32;
    let dy = (dy as f32 / dis).round() as i32;
    Object::move_by(id, dx, dy, map, objects);
}

pub fn ai_take_turn(monster_id: usize, tcod: &Tcod, game: &Game, objects: &mut [Object]) {
    let (monster_x, monster_y) = objects[monster_id].pos();
    if tcod.fov.is_in_fov(monster_x, monster_y) {
        if objects[monster_id].distance_to(&objects[PLAYER]) >= 2.0 {
            let (player_x, player_y) = objects[PLAYER].pos();
            move_towards(monster_id, player_x, player_y, &game.map, objects);
        } else if objects[PLAYER].fighter.map_or(false, |f| f.hp > 0) {
            let (player, monster) = mut_two(PLAYER, monster_id, objects);
            monster.attack(player);
        }
    }
}
