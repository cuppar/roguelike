use tcod::{
    colors::WHITE,
    console::{blit, Offscreen, Root},
    Console,
};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub const INVENTORY_WIDTH:i32 = 50;

pub fn menu<T>(header: &str, options: &[T], width: i32, root: &mut Root) -> Option<usize>
where
    T: AsRef<str>,
{
    assert!(
        options.len() <= 26,
        "Cannot have a menu with more than 26 options."
    );
    let header_height = root.get_height_rect(0, 0, width, SCREEN_HEIGHT, header);
    let height = header_height + options.len() as i32;
    let mut window = Offscreen::new(width, height);
    window.set_default_foreground(WHITE);
    window.print_rect_ex(
        0,
        0,
        width,
        height,
        tcod::BackgroundFlag::None,
        tcod::TextAlignment::Left,
        header,
    );
    for (index, option_text) in options.iter().enumerate() {
        let menu_letter = (b'a' + index as u8) as char;
        let text = format!("({}) {}", menu_letter, option_text.as_ref());
        window.print_ex(
            0,
            header_height + index as i32,
            tcod::BackgroundFlag::None,
            tcod::TextAlignment::Left,
            text,
        );
    }
    let x = SCREEN_WIDTH / 2 - width / 2;
    let y = SCREEN_HEIGHT / 2 - height / 2;
    blit(&window, (0, 0), (width, height), root, (x, y), 1.0, 0.7);

    root.flush();
    let key = root.wait_for_keypress(true);
    if key.printable.is_alphabetic() {
        let index = key.printable.to_ascii_lowercase() as usize - 'a' as usize;

        if index < options.len() {
            Some(index)
        } else {
            None
        }
    } else {
        None
    }
}
