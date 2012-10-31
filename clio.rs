extern mod amulet;

use option::{None, Option, Some};
use rand::task_rng;

use amulet::ll;

fn main() {
    let map = generate_map();

    let window = amulet::ll::init_screen();

    loop {
        draw_map(window, map);
        match window.read_key() {
            ll::Character('q') => return,
            ll::SpecialKey(ll::KEY_UP) => { move_player(map, 0, -1); }
            ll::SpecialKey(ll::KEY_DOWN) => { move_player(map, 0, 1); }
            ll::SpecialKey(ll::KEY_LEFT) => { move_player(map, -1, 0); }
            ll::SpecialKey(ll::KEY_RIGHT) => { move_player(map, 1, 0); }
            _ => {},
        }
    }
}

fn draw_map(window: &amulet::ll::Window, map: @Map) {
    for uint::range(0, map.grid.len()) |x| {
        for uint::range(0, map.grid[x].len()) |y| {
            let tile = map.grid[x][y];
            window.mv(y, x);
            let disp = match tile.creature {
                Some(creature) => creature.proto.display,
                None => tile.architecture.proto.display,
            };
            window.print(fmt!("%c", disp));
        }
    }

    // Stick the cursor on the player
    match map.player.position {
        (x, y) => window.mv(y, x),
    }

    window.repaint();
}

fn move_player(map: &Map, dx: int, dy: int) {
    let (x, y) = map.player.position;
    let new_x = (x as int + dx) as uint;
    let new_y = (y as int + dy) as uint;
    // TODO point type?
    // TODO check in bounds...
    let target_tile = map.grid[new_x][new_y];
    if target_tile.architecture.is_passable() && target_tile.creature.is_none() {
        map.player.position = (new_x, new_y);
        map.grid[x][y].creature = None;
        map.grid[new_x][new_y].creature = Some(map.player);
    }
}

////////////////////////////////////////////////////////////////////////////////

struct Map {
    mut grid: ~[~[@Tile]],
    mut player: @Entity,
    // TODO explicit size
}
fn generate_map() -> @Map {
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
            @Tile{ architecture: @Entity{ proto: proto, position: (x, y) }, creature: None }
        })
    });

    let player_x = task_rng().gen_uint_range(room_x + 1, room_x + room_width - 1);
    let player_y = task_rng().gen_uint_range(room_y + 1, room_y + room_height - 1);
    let player = @Entity{ proto: &PLAYER, position: (player_x, player_y) };
    grid[player_x][player_y].creature = Some(player);

    return @Map{ grid: grid, player: player };
}

impl Map {
}


////////////////////////////////////////////////////////////////////////////////

struct Tile {
    mut architecture: @Entity,
    mut creature: Option<@Entity>,
}

struct Entity {
    proto: &Prototype,
    mut position: (uint, uint),
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

