use super::object::*;
use super::render::*;
use super::Game;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

pub fn handle_keys(tcod: &mut Tcod, game: &mut Game, objects: &mut [Object]) -> PlayerAction {
    use tcod::input::KeyCode::*;
    use tcod::input::*;
    use PlayerAction::*;

    let player_alive = objects[PLAYER].alive;

    match (tcod.key, tcod.key.text(), player_alive) {
        (
            Key {
                code: Char,
                printable: 'w',
                ..
            },
            _,
            true,
        ) => {
            player_move_or_attack(0, -1, game, objects);
            TookTurn
        }
        (
            Key {
                code: Char,
                printable: 's',
                ..
            },
            _,
            true,
        ) => {
            player_move_or_attack(0, 1, game, objects);
            TookTurn
        }
        (
            Key {
                code: Char,
                printable: 'a',
                ..
            },
            _,
            true,
        ) => {
            player_move_or_attack(-1, 0, game, objects);
            TookTurn
        }
        (
            Key {
                code: Char,
                printable: 'd',
                ..
            },
            _,
            true,
        ) => {
            player_move_or_attack(1, 0, game, objects);
            TookTurn
        }
        (Key { code: Up, .. }, _, true) => {
            player_move_or_attack(0, -1, game, objects);
            TookTurn
        }
        (Key { code: Down, .. }, _, true) => {
            player_move_or_attack(0, 1, game, objects);
            TookTurn
        }
        (Key { code: Left, .. }, _, true) => {
            player_move_or_attack(-1, 0, game, objects);
            TookTurn
        }
        (Key { code: Right, .. }, _, true) => {
            player_move_or_attack(1, 0, game, objects);
            TookTurn
        }
        (
            Key {
                code: Enter,
                ctrl: true,
                ..
            },
            ..,
        ) => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
            DidntTakeTurn
        }
        (Key { code: Escape, .. }, ..) => Exit,
        _ => DidntTakeTurn,
    }
}
