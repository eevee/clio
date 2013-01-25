use amulet::ll;

use interface::terminal::make_terminal_interface;
use world;

fn main() {
    let world = world::new_game();

    let interface = make_terminal_interface();
    interface.message("welcome!");

    world.run(interface);
}
