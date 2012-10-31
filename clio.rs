extern mod amulet;

use option::{None, Option, Some};
use rand::task_rng;

fn main() {
    let map = generate_map();

    let window = amulet::ll::init_screen();

    for map.grid.eachi |x, col| {
        for col.eachi |y, tile| {
            window.mv(y, x);
            let disp = match tile.creature {
                Some(creature) => creature.proto.display,
                None => tile.architecture.proto.display,
            };
            window.print(fmt!("%c", disp));
        }
    }
    match map.player.position {
        (x, y) => window.mv(y, x),
    }

    window.repaint();
    window.getch();
}

////////////////////////////////////////////////////////////////////////////////

struct Map {
    mut grid: ~[~[Tile]],
    mut player: @Entity,
}
fn generate_map() -> ~Map {
    let width = 80;
    let height = 24;

    let room_width = task_rng().gen_uint_range(5, width + 1);
    let room_x = task_rng().gen_uint_range(0, width - room_width + 1);
    let room_height = task_rng().gen_uint_range(5, height + 1);
    let room_y = task_rng().gen_uint_range(0, height - room_height + 1);

    let mut grid = vec::from_fn(width, |x| {
        vec::from_fn(height, |y| {
            let proto =
                if x < room_x || y < room_y || x >= room_x + room_width  || y >= room_y + room_height {
                    // Outside the room
                    &SOLID_ROCK
                }
                else if x == room_x || y == room_y || x == room_x + room_width - 1 || y == room_y + room_height - 1 {
                    // Edge of the room
                    &WALL
                }
                else {
                    // Inside of the room
                    &FLOOR
                }
            ;
            Tile{ architecture: @Entity{ proto: proto, position: (x, y) }, creature: None }
        })
    });

    let player_x = task_rng().gen_uint_range(room_x + 1, room_x + room_width - 1);
    let player_y = task_rng().gen_uint_range(room_y + 1, room_y + room_height - 1);
    let player = @Entity{ proto: &PLAYER, position: (player_x, player_y) };
    grid[player_x][player_y].creature = Some(player);

    return ~Map{ grid: grid, player: player };
}

impl Map {
}


////////////////////////////////////////////////////////////////////////////////

struct Tile {
    architecture: @Entity,
    creature: Option<@Entity>,
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


const SOLID_ROCK: Prototype = Prototype{ display: ' ', passable: false };
const WALL: Prototype = Prototype{ display: '▒', passable: false };
const FLOOR: Prototype = Prototype{ display: ' ', passable: true };
const PLAYER: Prototype = Prototype{ display: '☻', passable: false };

