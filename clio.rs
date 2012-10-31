extern mod amulet;

use option::{None, Option, Some};

fn main() {
    let mut map = vec::from_fn(10, |i| {
        vec::from_fn(10, |j| {
            if i == 0 || i == 9 || j == 0 || j == 9 {
                Tile{ architecture: ~Entity{ proto: &WALL, position: (i, j) }, creature: None }
            }
            else {
                Tile{ architecture: ~Entity{ proto: &FLOOR, position: (i, j) }, creature: None }
            }
        })
    });

    map[3][3].creature = Some(~Entity{ proto: &PLAYER, position: (3, 3) });

    let window = amulet::ll::init_screen();

    for map.eachi |i, row| {
        for row.eachi |j, tile| {
            window.mv(i, j);
            let disp = match tile.creature {
                Some(creature) => creature.proto.display,
                None => tile.architecture.proto.display,
            };
            window.print(fmt!("%c", disp));
        }
    }
    window.mv(3, 3);

    window.repaint();
    window.getch();
}

////////////////////////////////////////////////////////////////////////////////

/*
struct World {
}
*/

////////////////////////////////////////////////////////////////////////////////

struct Tile {
    architecture: ~Entity,
    creature: Option<~Entity>,
}

struct Entity {
    proto: &Prototype,
    position: (uint, uint),
}
impl Entity {
    // PHYSICS
    fn is_passable() -> bool {
        return self.proto.passable;
    }
}


struct Prototype {
    display: char,
    passable: bool,
}


const WALL: Prototype = Prototype{ display: '#', passable: false };
const FLOOR: Prototype = Prototype{ display: ' ', passable: true };
const PLAYER: Prototype = Prototype{ display: '@', passable: false };

