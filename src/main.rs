use tcod::colors::*;
use tcod::console::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
}

struct Player {
    x: i32,
    y: i32,
}
/// return true to exit game
fn handle_keys(tcod: &mut Tcod, player_x: &mut i32, player_y: &mut i32) -> bool {
    use tcod::input::KeyCode::*;
    use tcod::input::*;

    let key: Key = tcod.root.wait_for_keypress(true);
    match key {
        Key { code: Up, .. } => *player_y -= 1,
        Key { code: Down, .. } => *player_y += 1,
        Key { code: Left, .. } => *player_x -= 1,
        Key { code: Right, .. } => *player_x += 1,
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

fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();
    let mut tcod = Tcod { root };
    tcod::system::set_fps(LIMIT_FPS);
    let mut player = Player {
        x: SCREEN_WIDTH / 2,
        y: SCREEN_HEIGHT / 2,
    };
    while !tcod.root.window_closed() {
        tcod.root.set_default_foreground(WHITE);
        tcod.root.clear();
        tcod.root
            .put_char(player.x, player.y, '@', BackgroundFlag::None);
        tcod.root.flush();
        // handle user input
        let exit = handle_keys(&mut tcod, &mut player.x, &mut player.y);
        if exit {
            break;
        }
    }
}
