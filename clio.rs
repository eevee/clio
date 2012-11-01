extern mod amulet;

use option::{None, Option, Some};
use rand::task_rng;

use amulet::ll;

fn main() {
    let window = amulet::ll::init_screen();
    window.hide_cursor();

    // NOTE: this has to be after the init_screen call, because curses is
    // stupid, and initializing the screen wrecks all the color pairs.
    let map = generate_map();

    // Create a persistent status area
    let status_window = amulet::ll::new_window(0, 0, 0, map.width());
    let message_window = amulet::ll::new_window(0, 80, 24, 0);
    message_window.print("welcome!");

    loop {
        // Display
        draw_map(window, map);

        status_window.clear();
        status_window.print(fmt!("⌛ %u", map.clock));
        let tile = map.player_tile();
        if tile.items.len() > 0 {
            status_window.mv(1, 0);
            status_window.print("you see here:");
            status_window.mv(2, 4);
            for uint::range(0, tile.items.len()) |i| {
                status_window.print("an item");
            }
        }
        status_window.repaint();

        message_window.repaint();

        // Input loop
        match window.read_key() {
            ll::Character('q') => return,
            ll::SpecialKey(ll::KEY_UP) => { move_player(map, 0, -1); }
            ll::SpecialKey(ll::KEY_DOWN) => { move_player(map, 0, 1); }
            ll::SpecialKey(ll::KEY_LEFT) => { move_player(map, -1, 0); }
            ll::SpecialKey(ll::KEY_RIGHT) => { move_player(map, 1, 0); }
            _ => {},
        }

        // TODO only advance clock if the player actually does something
        map.clock += 1;
    }
}

fn draw_map(window: &amulet::ll::Window, map: @Map) {
    for uint::range(0, map.width()) |x| {
        for uint::range(0, map.height()) |y| {
            let tile = map.grid[x][y];
            window.mv(y, x);
            let proto = match tile.creature {
                Some(creature) => creature.proto,
                None => {
                    if tile.items.len() > 0 {
                        tile.items[0].proto
                    }
                    else {
                        tile.architecture.proto
                    }
                }
            };
            window.attrprint(fmt!("%c", proto.display), proto.style);
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
    size: (uint, uint),
    mut grid: ~[~[@Tile]],
    mut player: @Entity,

    // TODO this goes on the world, really.
    mut clock: uint,
}
fn generate_map() -> @Map {
    // TODO oh fuck, these can't be constants if they have generated Styles in them.  WHOOPS
    let SOLID_ROCK: @Prototype = @Prototype{ display: ' ', style: ll::Style(), passable: false };
    let WALL: @Prototype = @Prototype{ display: '▒', style: ll::Style().fg(8), passable: false };
    let FLOOR: @Prototype = @Prototype{ display: '·', style: ll::Style().fg(8), passable: true };
    let PLAYER: @Prototype = @Prototype{ display: '☻', style: ll::Style().fg(4), passable: false };
    let SCROLL: @Prototype = @Prototype{ display: '?', style: ll::Style().bold(), passable: true };
    let ENEMY: @Prototype = @Prototype{ display: 'a', style: ll::Style().fg(1).bold(), passable: true };

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
                    SOLID_ROCK
                }
                else if x == room_x || y == room_y || x == room_x + room_width - 1 || y == room_y + room_height - 1 {
                    // Edge of the room
                    WALL
                }
                else {
                    // Inside of the room
                    FLOOR
                }
            ;
            @Tile{ architecture: @Entity{ proto: proto, position: (x, y) }, creature: None, items: ~[] }
        })
    });

    let player_x = task_rng().gen_uint_range(room_x + 1, room_x + room_width - 1);
    let player_y = task_rng().gen_uint_range(room_y + 1, room_y + room_height - 1);
    let player = @Entity{ proto: PLAYER, position: (player_x, player_y) };
    grid[player_x][player_y].creature = Some(player);

    loop {
        let enemy_x = task_rng().gen_uint_range(room_x + 1, room_x + room_width - 1);
        let enemy_y = task_rng().gen_uint_range(room_y + 1, room_y + room_height - 1);
        if enemy_x == player_x && enemy_y == player_y {
            loop;
        }
        let enemy = @Entity{ proto: ENEMY, position: (enemy_x, enemy_y) };
        grid[enemy_x][enemy_y].creature = Some(enemy);
        break;
    }

    let scroll_x = task_rng().gen_uint_range(room_x + 1, room_x + room_width - 1);
    let scroll_y = task_rng().gen_uint_range(room_y + 1, room_y + room_height - 1);
    let scroll = @Entity{ proto: SCROLL, position: (scroll_x, scroll_y) };
    grid[scroll_x][scroll_y].items.push(scroll);

    return @Map{ size: (width, height), grid: grid, player: player, clock: 0 };
}

impl Map {
    fn width() -> uint {
        let (width, _height) = self.size;
        return width;
    }
    fn height() -> uint {
        let (_width, height) = self.size;
        return height;
    }

    fn player_tile() -> @Tile {
        let (x, y) = self.player.position;
        return self.grid[x][y];
    }
}


////////////////////////////////////////////////////////////////////////////////

struct Tile {
    mut architecture: @Entity,
    mut creature: Option<@Entity>,
    mut items: ~[@Entity],
}

struct Entity {
    proto: @Prototype,
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
    style: ll::Style,
    passable: bool,
}

