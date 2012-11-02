use option::{None, Option, Some};
use rand::task_rng;

use amulet::ll;

use entity;
use entity::Entity;
use entity::OnFloor;
use entity::Prototype;
use geometry::Point;
use geometry::Size;
use world::Map;
use world::Tile;

// -----------------------------------------------------------------------------
// The RNG god.

/** Roll XdY, except assume the dice start at zero. */
fn roll(trials: uint, sides: uint) -> uint {
    let rng = task_rng();
    let mut rv: uint = 0;
    do trials.times {
        rv += rng.gen_uint_range(0, sides);
        true
    }
    return rv;
}

/** Pick a random number from 0 to (n-1), with a heavy normal-distribution bias
 * towards the middle. */
fn roll_normalish(n: uint) -> uint {
    let trials = 4;
    return roll(trials, n / trials);
}

/** Flip a coin. */
fn flip() -> bool {
    return roll(1, 2) == 0;
}

// -----------------------------------------------------------------------------
// "Fractor" is the agent noun form of "fractal".  :)

trait Fractor {
}

trait Feature {
}

/** A Room is surrounded on all sides by walls and contains some nonzero amount
 * of internal floor. */
//struct Room {
//}

struct Region {
    x: uint,
    y: uint,
    width: uint,
    height: uint,
}
impl Region {
    fn pick_point() -> Point {
        // Pick a point, omitting the walls
        // TODO actually that's kinda room-specific, huh.
        return Point{
            x: task_rng().gen_uint_range(self.x + 1, self.x + self.width - 1) as int,
            y: task_rng().gen_uint_range(self.y + 1, self.y + self.height - 1) as int,
        };
    }

    /** Split a Region into two halves, vertically, at a random point near-ish
     * the middle. */
    fn split_vert() -> ~[Region] {
        // TODO this should really really not span the /entire/ width
        let split_point = roll_normalish(self.width);
        return ~[
            Region{ width: split_point, ..self },
            Region{ x: split_point + self.x, width: self.width - split_point, ..self }
        ];
    }

    // TODO should return Room but i don't know how to do that since there is
    // no subclassing
    fn add_room() -> Region {
        let width = roll_normalish(self.width);
        let height = roll_normalish(self.height);
        let x = self.x + roll_normalish(self.width - width);
        let y = self.y + roll_normalish(self.height - height);

        return Region{ x: x, y: y, width: width, height: height };
    }

    fn draw_onto(canvas: &[~[@Tile]]) {
        // Top and bottom walls
        for uint::range(self.x, self.x + self.width) |x| {
            canvas[x][self.y].architecture = entity::WALL.make_entity();
            canvas[x][self.y + self.height - 1].architecture = entity::WALL.make_entity();
        }

        // Left and right walls (don't overwrite corners!)
        for uint::range(self.y + 1, self.y + self.height - 1) |y| {
            canvas[self.x][y].architecture = entity::WALL.make_entity();
            canvas[self.x + self.width - 1][y].architecture = entity::WALL.make_entity();
        }

        // Interior
        for uint::range(self.x + 1, self.x + self.width - 1) |x| {
            for uint::range(self.y + 1, self.y + self.height - 1) |y| {
                canvas[x][y].architecture = entity::FLOOR.make_entity();
            }
        }
    }

    fn place_creature(proto: &static/Prototype, canvas: &[~[@Tile]]) -> @Entity {
        let entity = proto.make_entity();

        // TODO unlikely, but possible infinite loop
        loop {
            let point = self.pick_point();
            if canvas[point.x][point.y].creature.is_none() {
                entity.location = OnFloor(point);
                canvas[point.x][point.y].creature = Some(entity);
                return entity;
            }
        }
    }
}

pub fn generate_map() -> @Map {
    let region = Region{ x: 0, y: 0, width: 80, height: 24 };

    // TODO the api for actually drawing kinda sucks and does a bit of
    // overwriting

    let mut canvas = vec::from_fn(region.width, |x| {
        vec::from_fn(region.height, |y| {
            let entity = entity::ROCKFACE.make_entity();
            entity.location = OnFloor(Point{ x: x as int, y: y as int });
            @Tile{ architecture: entity, creature: None, items: ~[] }
        })
    });

    let regions = region.split_vert();

    let mut rooms = do regions.map |region| {
        let room = region.add_room();

        room.draw_onto(canvas);

        room
    };

    let mut left_room = rooms[0];
    let mut right_room = rooms[1];

    // Connect the rooms...  oh dear.
    let mut start_y = task_rng().gen_uint_range(left_room.y + 1, left_room.y + left_room.height - 1);
    let mut end_y = task_rng().gen_uint_range(right_room.y + 1, right_room.y + right_room.height - 1);
    let mid_x = task_rng().gen_uint_range(left_room.x + left_room.width, right_room.x + 1);

    // cut through the walls!
    for uint::range(left_room.x + left_room.width - 1, right_room.x + 1) |x| {
        if x <= mid_x {
            canvas[x][start_y].architecture = entity::PASSAGE.make_entity();
        }
        if x >= mid_x {
            canvas[x][end_y].architecture = entity::PASSAGE.make_entity();
        }
    }
    if start_y > end_y {
        start_y <-> end_y;
    }
    if end_y - start_y > 1 {
        for uint::range(start_y + 1, end_y) |y| {
            canvas[mid_x][y].architecture = entity::PASSAGE.make_entity();
        }
    }

    // Place some things
    if flip() {
        left_room <-> right_room;
    }

    let player = left_room.place_creature(&entity::PLAYER, canvas);
    right_room.place_creature(&entity::ENEMY, canvas);
    let point = right_room.pick_point();
    let scroll = entity::SCROLL.make_entity();
    scroll.location = OnFloor(point);
    canvas[point.x][point.y].items.push(scroll);

    return @Map{ size: Size{ width: region.width, height: region.height }, grid: canvas, player: player };
}
