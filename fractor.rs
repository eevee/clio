use option::{None, Option, Some};
use rand::task_rng;

use amulet::ll;

use entity::Entity;
use entity::Prototype;
use world::Map;
use world::Tile;

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
