use entity::Action;
use world::World;

mod terminal;

/** Something that adapts the game core to a human being; performs I/O. */
// NB: These methods take `&mut self` because output tends to involve writing
pub trait Interface {
    // Input

    /** Prompt the user for the next action to take.  Should probably block. */
    fn next_action(&mut self, world: &World) -> ~Action:'static;


    // Output

    /** Redraw the game field. */
    fn redraw(&mut self, world: &World);

    /** Show a game message to the player. */
    fn message(&mut self, s: &str);

    /** End the game */
    fn end(&mut self);
}
