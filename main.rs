use amulet::ll;

use interface::Interface;
use interface::terminal::make_terminal_interface;
use world;

#[main]
pub fn main() {
    let world = world::new_game();

    let mut interface = make_terminal_interface();
    interface.message("welcome!");

    // TODO no obvious way to pass ~Foo as &FooTrait; maybe make_*_interface
    // shouldn't return a pointer at all, for starters.  or should return a
    // pre-traited thing?
    world.run(&mut interface as &mut Interface);
}
