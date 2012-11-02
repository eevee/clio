use option::{None, Option, Some};
use rand::task_rng;

use amulet::ll;

use entity;
use entity::Entity;
use entity::OnFloor;
use entity::Prototype;
use world::Map;
use world::Tile;


fn generate_map() -> @Map {
    let width = 80;
    let height = 24;

    let room_width = task_rng().gen_uint_range(5, width + 1);
    let room_x = task_rng().gen_uint_range(0, width - room_width + 1);
    let room_height = task_rng().gen_uint_range(5, height + 1);
    let room_y = task_rng().gen_uint_range(0, height - room_height + 1);

    let mut grid = vec::from_fn(width, |x| {
        vec::from_fn(height, |y| {
            let entity =
                if x < room_x || y < room_y || x >= room_x + room_width  || y >= room_y + room_height {
                    // Outside the room
                    entity::ROCKFACE.make_entity()
                }
                else if x == room_x || y == room_y || x == room_x + room_width - 1 || y == room_y + room_height - 1 {
                    // Edge of the room
                    entity::WALL.make_entity()
                }
                else {
                    // Inside of the room
                    entity::FLOOR.make_entity()
                }
            ;
            entity.location = entity::OnFloor(x, y);
            @Tile{ architecture: entity, creature: None, items: ~[] }
        })
    });

    let player_x = task_rng().gen_uint_range(room_x + 1, room_x + room_width - 1);
    let player_y = task_rng().gen_uint_range(room_y + 1, room_y + room_height - 1);
    let player = entity::PLAYER.make_entity();
    player.location = entity::OnFloor(player_x, player_y);
    grid[player_x][player_y].creature = Some(player);

    loop {
        let enemy_x = task_rng().gen_uint_range(room_x + 1, room_x + room_width - 1);
        let enemy_y = task_rng().gen_uint_range(room_y + 1, room_y + room_height - 1);
        if enemy_x == player_x && enemy_y == player_y {
            loop;
        }
        let enemy = entity::ENEMY.make_entity();
        enemy.location = OnFloor(enemy_x, enemy_y);
        grid[enemy_x][enemy_y].creature = Some(enemy);
        break;
    }

    let scroll_x = task_rng().gen_uint_range(room_x + 1, room_x + room_width - 1);
    let scroll_y = task_rng().gen_uint_range(room_y + 1, room_y + room_height - 1);
    let scroll = entity::SCROLL.make_entity();
    scroll.location = OnFloor(scroll_x, scroll_y);
    grid[scroll_x][scroll_y].items.push(scroll);

    return @Map{ size: (width, height), grid: grid, player: player, clock: 0 };
}
