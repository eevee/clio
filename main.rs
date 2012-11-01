use amulet::ll;

use display::draw_map;
use fractor::generate_map;

fn main() {
    let window = amulet::ll::init_screen();
    window.hide_cursor();

    // NOTE: this has to be after the init_screen call, because curses is
    // stupid, and initializing the screen wrecks all the color pairs.
    let map = generate_map();

    // Create a persistent status area
    let status_window = amulet::ll::new_window(0, 0, 0, map.width());
    let message_window = amulet::ll::new_window(0, 80, 24, 0);
    message_window.print("welcome!");

    loop {
        // Display
        draw_map(window, map);

        status_window.clear();
        status_window.print(fmt!("⌛ %u", map.clock));
        let tile = map.player_tile();
        if tile.items.len() > 0 {
            status_window.mv(1, 0);
            status_window.print("you see here:");
            status_window.mv(2, 4);
            for uint::range(0, tile.items.len()) |i| {
                status_window.print("an item");
            }
        }
        status_window.repaint();

        message_window.repaint();

        // Input loop
        match window.read_key() {
            ll::Character('q') => return,
            ll::SpecialKey(ll::KEY_UP) => { map.move_player(0, -1); }
            ll::SpecialKey(ll::KEY_DOWN) => { map.move_player(0, 1); }
            ll::SpecialKey(ll::KEY_LEFT) => { map.move_player(-1, 0); }
            ll::SpecialKey(ll::KEY_RIGHT) => { map.move_player(1, 0); }
            _ => {},
        }

        // TODO only advance clock if the player actually does something
        map.clock += 1;
    }
}
