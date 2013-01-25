mod terminal;

use entity::Action;
use world::World;

/** Something that adapts the game core to a human being; performs I/O. */
pub trait Interface {
    // Input

    /** Prompt the user for the next action to take.  Should probably block. */
    fn next_action(world: &World) -> Action;


    // Output

    /** Redraw the game field. */
    fn redraw(world: &World);

    /** Show a game message to the player. */
    fn message(s: &str);

    /** End the game */
    fn end();
}
