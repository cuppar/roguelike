use super::object::*;
use super::render::*;
use super::Game;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    Replay,
    TookTurn,
    DidntTakeTurn,
    Exit,
}

pub fn handle_keys(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) -> PlayerAction {
    use tcod::input::KeyCode::*;
    use tcod::input::*;
    use PlayerAction::*;

    let key: Key = tcod.root.wait_for_keypress(true);
    let player_alive = objects[PLAYER].alive;
    match (key, key.text(), player_alive) {
        (
            Key {
                code: Char,
                printable: 'g',
                ..
            },
            _,
            true,
        ) => {
            let item_id = objects
                .iter()
                .position(|o| o.pos() == objects[PLAYER].pos() && o.item.is_some());
            if let Some(item_id) = item_id {
                pick_item_up(item_id, game, objects);
            }
            DidntTakeTurn
        }
        (
            Key {
                code: Char,
                printable: 'i',
                ..
            },
            _,
            true,
        ) => {
            let inventory_index = inventory_menu(
                &game.inventory,
                "Press the key next to an item to use it, or any other to cancel.\n",
                &mut tcod.root,
            );
            if let Some(inventory_index) = inventory_index {
                use_item(inventory_index, tcod, game, objects);
            }
            TookTurn
        }
        (
            Key {
                code: Char,
                printable: 'y',
                ..
            },
            _,
            false,
        ) => Replay,
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
