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
    message_window.print("welcome!\n");

    loop {
        // Display
        draw_map(window, map);

        status_window.clear();
        status_window.print(fmt!("⌛ %u", map.clock));
        status_window.mv(1, 0);
        status_window.print("inventory: ");
        for uint::range(0, map.player.contents.len()) |i| {
            let proto = map.player.contents[i].proto;
            status_window.attrprint(fmt!("%c", proto.display), proto.style);
        }
        let tile = map.player_tile();
        if tile.items.len() > 0 {
            status_window.mv(2, 0);
            status_window.print("you see here:");
            status_window.mv(3, 4);
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
            ll::Character(',') => {
                let player_tile = map.player_tile();
                if player_tile.items.len() > 0 {
                    map.player.contents += player_tile.items;
                    player_tile.items = ~[];
                }
                else {
                    message_window.print("nothing here...\n");
                }
            }
            _ => {},
        }

        // TODO only advance clock if the player actually does something
        map.clock += 1;
    }
}
