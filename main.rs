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
            ll::SpecialKey(ll::KEY_UP) => { map.move_entity(map.player, 0, -1); }
            ll::SpecialKey(ll::KEY_DOWN) => { map.move_entity(map.player, 0, 1); }
            ll::SpecialKey(ll::KEY_LEFT) => { map.move_entity(map.player, -1, 0); }
            ll::SpecialKey(ll::KEY_RIGHT) => { map.move_entity(map.player, 1, 0); }
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

        // Now let every object on the board advance in time.
        // TODO obviously the clock will be a little more fine-grained than
        // this.
        // TODO not only creatures need time...
        // TODO invert this so the game itself handles player actions as well
        // TODO and only advance the clock if the player performs a real action
        game.advance_clock(display);
    }
}
