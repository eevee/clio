use amulet::ll;

use display;
use world;

fn main() {
    let game = world::new_game();
    let map = game.map;

    let display = display::TerminalDisplay(game);
    display.message("welcome!");

    loop {
        // Display
        display.update();

        // Input loop
        match display.main_window.read_key() {
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
                    display.message("nothing here...\n");
                }
            }
            _ => {},
        }

        // TODO only advance clock if the player actually does something
        map.clock += 1;
    }
}
