use std::borrow::ref_eq;
use std::managed;
use std::option::{Option, None, Some};
use std::rand::RngUtil;  // for gen_uint_range
use std::rand::task_rng;

use amulet::ll;
use amulet::ll::Style;

use geometry::Offset;
use geometry::Point;
use interface::Interface;
use world::World;

// -----------------------------------------------------------------------------
// Prototypes: templates for entities

// TODO should probably distinguish somehow between architecture, creatures,
// and flooring in a static way

// Architecture
pub static ROCKFACE: Prototype = Prototype{
    display: ' ', style: Style{ is_bold: false, is_underline: false, fg_color: -1, bg_color: -1 },
    passable: false,
    unspeed: 0,
};
pub static WALL: Prototype = Prototype{
    display: '▒', style: Style{ is_bold: false, is_underline: false, fg_color: 8, bg_color: -1 },
    passable: false,
    unspeed: 0,
};
pub static FLOOR: Prototype = Prototype{
    display: '·', style: Style{ is_bold: false, is_underline: false, fg_color: 8, bg_color: -1 },
    passable: true,
    unspeed: 0,
};
pub static PASSAGE: Prototype = Prototype{
    display: '░', style: Style{ is_bold: false, is_underline: false, fg_color: 8, bg_color: -1 },
    passable: true,
    unspeed: 0,
};

// Creatures
// TODO 'player' is not a species
pub static PLAYER: Prototype = Prototype{
    display: '☻', style: Style{ is_bold: false, is_underline: false, fg_color: 4, bg_color: -1 },
    passable: false,
    unspeed: 48,
};
pub static ENEMY: Prototype = Prototype{
    display: 'a', style: Style{ is_bold: true, is_underline: false, fg_color: 1, bg_color: -1 },
    passable: true,
    unspeed: 72,
};

// Objects
pub static SCROLL: Prototype = Prototype{
    display: '?', style: Style{ is_bold: true, is_underline: false, fg_color: -1, bg_color: -1 },
    passable: true,
    unspeed: 0,
};


pub struct Prototype {
    display: char,
    style: ll::Style,
    passable: bool,

    /// Base amount of time that an action takes, in tics
    unspeed: uint,
}
impl Prototype {
    /// Create a new entity from a prototype.
    pub fn make_entity(&'static self) -> @mut Entity {
        return @mut Entity{
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

pub struct Entity {
    proto: &'static Prototype,
    location: Location,
    contents: ~[@mut Entity],

    health: uint,

    spent_subtics: uint,
}
impl Entity {
    // PHYSICS
    pub fn is_passable(&self) -> bool {
        return self.proto.passable;
    }

    // BEHAVIOR
    pub fn act(@mut self, world: &mut World, interface: &@Interface) -> Option<~Action:'static> {
        let player = world.map.player;

        if managed::mut_ptr_eq(self, player) {
            return Some(interface.next_action(world));
        }

        let me_point = match self.location {
            OnFloor(pt) => pt,
            _ => fail!(~"todo"),
        };
        let player_point = match player.location {
            OnFloor(pt) => pt,
            _ => fail!(~"todo"),
        };

        let distance = player_point - me_point;

        // If the player is adjacent, attack!
        if distance.is_adjacent() {
            return Some(~AttackAction{ actor: self, target: player } as ~Action:'static);
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
pub trait Action {
    pub fn execute(&self, world: &mut World, interface: &@Interface);
}

/** `actor` strikes `target`. */
pub struct AttackAction {
    actor: @mut Entity,
    target: @mut Entity,
}
impl Action for AttackAction {
    fn execute(&self, world: &mut World, interface: &@Interface) {
        if ref_eq(self.target.proto, &PLAYER) {
            interface.message("it hits you!");
        }
        else if ref_eq(self.actor.proto, &PLAYER) {
            interface.message("you hit it!");
        }

        self.target.health -= 1;

        if self.target.health == 0 {
            if ref_eq(self.target.proto, &PLAYER) {
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
pub struct MoveAction {
    actor: @mut Entity,
    offset: Offset,
}
impl Action for MoveAction {
    fn execute(&self, world: &mut World, _interface: &@Interface) {
        world.map.move_entity(self.actor, self.offset.dx, self.offset.dy);
    }
}

/** `actor` does nothing. */
pub struct WaitAction {
    actor: @mut Entity,
}
impl Action for WaitAction {
    fn execute(&self, _world: &mut World, _interface: &@Interface) {
    }
}
