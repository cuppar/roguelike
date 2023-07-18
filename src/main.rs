use rand::Rng;
use std::vec;
use tcod::map::{FovAlgorithm, Map as FovMap};

use tcod::colors::*;
use tcod::console::*;

const SCREEN_WIDTH: i32 = 180;
const SCREEN_HEIGHT: i32 = 120;

const MAP_WIDTH: i32 = 180;
const MAP_HEIGHT: i32 = 115;

const LIMIT_FPS: i32 = 20;

const ROOM_MAX_SIZE: i32 = 12;
const ROOM_MIN_SIZE: i32 = 8;
const MAX_ROOMS: i32 = 100;

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 10;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LIGHT_WALL: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};
const COLOR_LIGHT_GROUND: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};

struct Tcod {
    root: Root,
    con: Offscreen,
    fov: FovMap,
}

/// return true to exit game
fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut Object) -> bool {
    use tcod::input::KeyCode::*;
    use tcod::input::*;

    let key: Key = tcod.root.wait_for_keypress(true);
    match key {
        Key {
            code: Char,
            printable: 'w',
            ..
        } => player.move_by(0, -1, game),
        Key {
            code: Char,
            printable: 's',
            ..
        } => player.move_by(0, 1, game),
        Key {
            code: Char,
            printable: 'a',
            ..
        } => player.move_by(-1, 0, game),
        Key {
            code: Char,
            printable: 'd',
            ..
        } => player.move_by(1, 0, game),
        Key { code: Up, .. } => player.move_by(0, -1, game),
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),
        Key {
            code: Enter,
            ctrl: true,
            ..
        } => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,
        _ => {}
    }
    false
}

#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { x, y, char, color }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        let new_x = self.x + dx;
        let new_y = self.y + dy;
        if new_x >= MAP_WIDTH || new_x < 0 || new_y >= MAP_HEIGHT || new_y < 0 {
            return;
        }

        if game.map[new_x as usize][new_y as usize].blocked {
            return;
        }
        self.x = new_x;
        self.y = new_y;
    }

    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}
impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }
    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

type Map = Vec<Vec<Tile>>;

struct Game {
    map: Map,
}

fn make_map(player: &mut Object) -> Map {
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
                player.x = new_x;
                player.y = new_y;
            } else {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rand::random() {
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }
            rooms.push(new_room);
        }
    }

    map
}

fn render_all(tcod: &mut Tcod, game: &Game, objects: &[Object], fov_recompute: bool) {
    if fov_recompute {
        let player = &objects[0];
        tcod.fov
            .compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
    }
    for object in objects {
        if tcod.fov.is_in_fov(object.x, object.y) {
            object.draw(&mut tcod.con);
        }
    }
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let visible = tcod.fov.is_in_fov(x, y);
            let wall = game.map[x as usize][y as usize].block_sight;
            let color = match (visible, wall) {
                (false, true) => COLOR_DARK_WALL,
                (false, false) => COLOR_DARK_GROUND,
                (true, true) => COLOR_LIGHT_WALL,
                (true, false) => COLOR_LIGHT_GROUND,
            };
            tcod.con
                .set_char_background(x, y, color, BackgroundFlag::Set);
        }
    }
    blit(
        &tcod.con,
        (0, 0),
        (MAP_WIDTH, MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
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

fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    use std::cmp::max;
    use std::cmp::min;

    for x in min(x1, x2)..=max(x1, x2) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    use std::cmp::max;
    use std::cmp::min;

    for y in min(y1, y2)..=max(y1, y2) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn main() {
    // objects and maps settings
    let player = Object::new(0, 0, '@', WHITE);
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', YELLOW);
    let mut objects = [player, npc];
    let game = Game {
        map: make_map(&mut objects[0]),
    };

    // console settings
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Roguelike!")
        .init();
    let mut tcod = Tcod {
        root,
        con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
        fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
    };
    tcod::system::set_fps(LIMIT_FPS);

    // set fov map
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(
                x,
                y,
                !game.map[x as usize][y as usize].block_sight,
                // !game.map[x as usize][y as usize].blocked,
                false,
            )
        }
    }

    let mut previous_player_position = (-1, -1);

    // main loop
    while !tcod.root.window_closed() {
        // clear prev frame
        tcod.con.clear();

        // render
        let fov_recompute = previous_player_position != (objects[0].x, objects[0].y);
        render_all(&mut tcod, &game, &objects, fov_recompute);
        tcod.root.flush();

        let player = &mut objects[0];
        previous_player_position = (player.x, player.y);

        // handle user input
        let exit = handle_keys(&mut tcod, &game, player);
        if exit {
            break;
        }
    }
}
