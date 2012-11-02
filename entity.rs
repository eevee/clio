use rand::task_rng;

use amulet::ll;
use amulet::ll::Style;

use geometry::Offset;
use geometry::Point;
use interface::Interface;
use world::World;

use option::{Option, None, Some};

// -----------------------------------------------------------------------------
// Prototypes: templates for entities

// TODO should probably distinguish somehow between architecture, creatures,
// and flooring in a static way

// Architecture
pub const ROCKFACE: Prototype = Prototype{
    display: ' ', style: Style{ is_bold: false, is_underline: false, fg_color: -1, bg_color: -1 },
    passable: false,
    unspeed: 0,
};
pub const WALL: Prototype = Prototype{
    display: '▒', style: Style{ is_bold: false, is_underline: false, fg_color: 8, bg_color: -1 },
    passable: false,
    unspeed: 0,
};
pub const FLOOR: Prototype = Prototype{
    display: '·', style: Style{ is_bold: false, is_underline: false, fg_color: 8, bg_color: -1 },
    passable: true,
    unspeed: 0,
};
pub const PASSAGE: Prototype = Prototype{
    display: '░', style: Style{ is_bold: false, is_underline: false, fg_color: 8, bg_color: -1 },
    passable: true,
    unspeed: 0,
};

// Creatures
// TODO 'player' is not a species
pub const PLAYER: Prototype = Prototype{
    display: '☻', style: Style{ is_bold: false, is_underline: false, fg_color: 4, bg_color: -1 },
    passable: false,
    unspeed: 48,
};
pub const ENEMY: Prototype = Prototype{
    display: 'a', style: Style{ is_bold: true, is_underline: false, fg_color: 1, bg_color: -1 },
    passable: true,
    unspeed: 72,
};

// Objects
pub const SCROLL: Prototype = Prototype{
    display: '?', style: Style{ is_bold: true, is_underline: false, fg_color: -1, bg_color: -1 },
    passable: true,
    unspeed: 0,
};


struct Prototype {
    display: char,
    style: ll::Style,
    passable: bool,

    /// Base amount of time that an action takes, in tics
    unspeed: uint,
}
impl &static/Prototype {
    /// Create a new entity from a prototype.
    fn make_entity() -> @Entity {
        return @Entity{
            proto: self,
            location: Nowhere,
            contents: ~[],

            // TODO this doesn't seem right.  not all objects have health.
            // component ahoy.  but same with `contents`, honestly.
            health: 5,

            // TODO does this belong to "behavior"?
            spent_subtics: 0,
        };
    }
}


// -----------------------------------------------------------------------------
// Entities: actual game objects

pub enum Location {
    Nowhere,
    OnFloor(Point),
    InContainer,
}

struct Entity {
    proto: &static/Prototype,
    mut location: Location,
    mut contents: ~[@Entity],

    mut health: uint,

    mut spent_subtics: uint,
}
impl @Entity {
    // PHYSICS
    fn is_passable() -> bool {
        return self.proto.passable;
    }

    // BEHAVIOR
    fn act(world: @World, interface: @Interface) -> Option<Action> {
        let player = world.map.player;

        if box::ptr_eq(self, player) {
            return Some(interface.next_action(world));
        }

        let me_point = match self.location {
            OnFloor(pt) => pt,
            _ => fail,
        };
        let player_point = match player.location {
            OnFloor(pt) => pt,
            _ => fail,
        };

        let distance = player_point - me_point;

        // If the player is adjacent, attack!
        if distance.is_adjacent() {
            return Some(AttackAction{ actor: self, target: player } as Action);
        }

        // Otherwise, approach!  Pick direction at random.
        let which = task_rng().gen_uint_range(0, distance.taxicab_length());
        if which < distance.x_mag() {
            world.map.move_entity(self, distance.x_dir(), 0);
        }
        else {
            world.map.move_entity(self, 0, distance.y_dir());
        }

        return None;
    }
}


// Actions...  oh boy.
trait Action {
    fn execute(world: &World, interface: @Interface);
}

/** `actor` strikes `target`. */
struct AttackAction {
    actor: @Entity,
    target: @Entity,
}
impl AttackAction: Action {
    fn execute(world: &World, interface: @Interface) {
        if ptr::ref_eq(self.target.proto, &PLAYER) {
            interface.message("it hits you!");
        }
        else if ptr::ref_eq(self.actor.proto, &PLAYER) {
            interface.message("you hit it!");
        }

        self.target.health -= 1;

        if self.target.health == 0 {
            if ptr::ref_eq(self.target.proto, &PLAYER) {
                interface.message("you die...");
                interface.end();
            }
            else {
                interface.message("it dies");
                world.map.remove_entity(self.target);
            }
        }
    }
}

/** `actor` moves by some amount. */
struct MoveAction {
    actor: @Entity,
    offset: Offset,
}
impl MoveAction: Action {
    fn execute(world: &World, _interface: @Interface) {
        world.map.move_entity(self.actor, self.offset.dx, self.offset.dy);
    }
}

/** `actor` does nothing. */
struct WaitAction {
    actor: @Entity,
}
impl WaitAction: Action {
    fn execute(_world: &World, _interface: @Interface) {
    }
}
