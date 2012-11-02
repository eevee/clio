use amulet::ll;
use amulet::ll::Style;

use display::TerminalDisplay;
use world::Game;

use option::{Option, None, Some};

// -----------------------------------------------------------------------------
// Prototypes: templates for entities

// TODO should probably distinguish somehow between architecture, creatures,
// and flooring in a static way

// Architecture
pub const ROCKFACE: Prototype = Prototype{
    display: ' ', style: Style{ is_bold: false, is_underline: false, fg_color: -1, bg_color: -1 },
    passable: false,
};
pub const WALL: Prototype = Prototype{
    display: '▒', style: Style{ is_bold: false, is_underline: false, fg_color: 8, bg_color: -1 },
    passable: false,
};
pub const FLOOR: Prototype = Prototype{
    display: '·', style: Style{ is_bold: false, is_underline: false, fg_color: 8, bg_color: -1 },
    passable: true,
};

// Creatures
pub const PLAYER: Prototype = Prototype{
    display: '☻', style: Style{ is_bold: false, is_underline: false, fg_color: 4, bg_color: -1 },
    passable: false,
};
pub const ENEMY: Prototype = Prototype{
    display: 'a', style: Style{ is_bold: true, is_underline: false, fg_color: 1, bg_color: -1 },
    passable: true,
};

// Objects
pub const SCROLL: Prototype = Prototype{
    display: '?', style: Style{ is_bold: true, is_underline: false, fg_color: -1, bg_color: -1 },
    passable: true,
};


struct Prototype {
    display: char,
    style: ll::Style,
    passable: bool,
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
        };
    }
}


// -----------------------------------------------------------------------------
// Entities: actual game objects

pub enum Location {
    Nowhere,
    OnFloor(uint, uint),
    InContainer,
}

struct Entity {
    proto: &static/Prototype,
    mut location: Location,
    mut contents: ~[@Entity],

    mut health: uint,
}
impl @Entity {
    // PHYSICS
    fn is_passable() -> bool {
        return self.proto.passable;
    }

    // BEHAVIOR
    fn act(game: @Game) -> Option<Action> {
        if ptr::ref_eq(self.proto, &PLAYER) {
            return None;
        }

        let player = game.map.player;
        let (my_x, my_y) = match self.location {
            OnFloor(x, y) => (x, y),
            _ => fail,
        };
        let (plr_x, plr_y) = match player.location {
            OnFloor(x, y) => (x, y),
            _ => fail,
        };

        let dx = plr_x as int - my_x as int;
        let dy = plr_y as int - my_y as int;

        // If the player is adjacent, attack!
        if dx * dy == 0 && int::abs(dx + dy) == 1 {
            return Some(AttackAction{ actor: self, target: player } as Action);
            //game.perform_attack(&self, player);
        }

        // Otherwise, approach
        if dx < 0 {
            game.map.move_entity(self, -1, 0);
        }
        else if dx > 0 {
            game.map.move_entity(self, 1, 0);
        }
        else if dy < 0 {
            game.map.move_entity(self, 0, -1);
        }
        else if dy > 0 {
            game.map.move_entity(self, 0, 1);
        }

        return None;
    }
}


// Actions...  oh boy.
trait Action {
    fn execute(game: &Game, display: &TerminalDisplay);
}

struct AttackAction {
    actor: @Entity,
    target: @Entity,
}
impl AttackAction: Action {
    fn execute(game: &Game, display: &TerminalDisplay) {
        self.target.health -= 1;
        if ptr::ref_eq(self.target.proto, &PLAYER) {
            display.message("ouch!");
        }
    }
}
