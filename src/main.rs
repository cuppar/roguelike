// 3party
use tcod::colors::*;
use tcod::console::*;
use tcod::map::Map as FovMap;

// own module
mod game;

use game::input::*;
use game::map::*;
use game::object::*;
use game::render::*;
use game::*;

const SCREEN_WIDTH: i32 = 160;
const SCREEN_HEIGHT: i32 = 100;

const LIMIT_FPS: i32 = 20;

fn main() {
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
        panel: Offscreen::new(SCREEN_WIDTH, PANEL_HEIGHT),
        fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
        key: Default::default(),
        mouse: Default::default(),
    };
    tcod::system::set_fps(LIMIT_FPS);

    'start_game: loop {
        // objects and maps settings
        let mut player = Object::new(0, 0, '@', "Cuppar", WHITE, true);
        player.fighter = Some(Fighter {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
            on_death: DeathCallback::Player,
        });
        player.alive = true;
        let mut objects = vec![player];
        let mut game = Game {
            map: make_map(&mut objects),
            messages: Messages::new(),
            inventory: Default::default(),
        };

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
        game.messages.add("Welcome!", RED);

        // main loop
        while !tcod.root.window_closed() {
            // This function has bug?
            // match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            //     Some((_, Event::Mouse(m))) => tcod.mouse = m,
            //     Some((_, Event::Key(k))) => tcod.key = k,
            //     _ => tcod.key = Default::default(),
            // }

            // clear prev frame
            tcod.con.clear();

            // render
            let fov_recompute = previous_player_position != objects[PLAYER].pos();
            render_all(&mut tcod, &mut game, &objects, fov_recompute);
            tcod.root.flush();

            previous_player_position = objects[PLAYER].pos();

            // handle user input
            let player_action = handle_keys(&mut tcod, &mut game, &mut objects);
            if player_action == PlayerAction::Exit {
                break 'start_game;
            }

            if objects[PLAYER].alive && player_action == PlayerAction::TookTurn {
                for id in 0..objects.len() {
                    // monster turn
                    if objects[id].ai.is_some() {
                        ai_take_turn(id, &tcod, &mut game, &mut objects);
                    }
                }
            }
            if !objects[PLAYER].alive {
                // user die
                if player_action == PlayerAction::Replay {
                    break;
                } else {
                    game.messages.add("replay? (y/esc)".to_string(), RED);
                }
            }
        }
    }
}
