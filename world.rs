use option::{Option, None, Some};

use interface::Interface;
use entity::Entity;
use entity::Nowhere;
use entity::OnFloor;
use fractor::generate_map;
use geometry::Offset;
use geometry::Point;
use geometry::Rectangle;
use geometry::Size;

pub struct Map {
    size: Size,
    mut grid: ~[~[@Tile]],
    mut player: @Entity,
}
impl Map {
    fn width() -> uint {
        return self.size.width;
    }
    fn height() -> uint {
        return self.size.height;
    }
    fn bounds() -> Rectangle {
        return Rectangle{
            topleft: Point{ x: 0, y: 0 },
            size: self.size,
        };
    }

    fn player_tile() -> @Tile {
        match self.player.location {
            OnFloor(point) => {
                return self.grid[point.x][point.y];
            }
            _ => fail,
        }
    }
    fn tile_relative(source: @Entity, offset: Offset) -> Option<@Tile> {
        let point = match source.location {
            OnFloor(pt) => pt,
            _ => fail,
        };
        let new_point = point + offset;

        if self.bounds().contains(&new_point) {
            return Some(self.grid[new_point.x][new_point.y])
        }
        else {
            return None;
        }
    }

    fn remove_entity(entity: @Entity) {
        match entity.location {
            OnFloor(copy point) => {
                let tile = self.grid[point.x][point.y];
                match tile.creature {
                    Some(copy creature) if managed::ptr_eq(entity, creature) => {
                        entity.location = Nowhere;
                        tile.creature = None;
                    }
                    _ => {
                        fail ~"Entity not where it claimed to be!";
                    }
                }
            }
            _ => {
                fail ~"Don't know how to remove this entity";
            }
        }
    }

    fn move_entity(entity: @Entity, dx: int, dy: int) {
        match entity.location {
            OnFloor(copy point) => {
                let new_x = point.x + dx;
                let new_y = point.y + dy;
                // TODO point type?
                // TODO check in bounds...
                let target_tile = self.grid[new_x][new_y];
                // TODO these checks should already be done by the time we get here
                if (copy target_tile.architecture).is_passable() && target_tile.creature.is_none() {
                    entity.location = OnFloor(Point{ x: new_x, y: new_y });
                    self.grid[new_x][new_y].creature <-> self.grid[point.x][point.y].creature;
                }
            }
            _ => fail ~"Can't move an entity that's not on the dungeon floor",
        }
    }
}

pub struct Tile {
    mut architecture: @Entity,
    mut creature: Option<@Entity>,
    mut items: ~[@Entity],
}


/// Number of subtics (speed units) per clock tick
const TIC_SIZE: uint = 48;

pub struct World {
    map: @Map,
    mut clock: uint,
}
pub fn new_game() -> @World {
    return @World{ map: generate_map(), clock: 0 };
}
impl World {
    /** Runs the game forever.  Ish. */
    fn run(@self, interface: @Interface) {
        // Draw the game world first
        interface.redraw(self);

        // Find everything in the world.
        // TODO extend this to letting every object in the world advance by one
        // clock tic; make it that generic componenty entry point of update()
        // (PS: that includes recursing into containers
        // TODO this will fuck up if objects are created or destroyed, the map
        // changes, etc.!  should possibly be an attribute of the map, so it
        // can be responsible for maintaining the order

        let mut actors: ~[@Entity] = ~[];
        for uint::range(0, self.map.width()) |x| {
            for uint::range(0, self.map.height()) |y| {
                match self.map.grid[x][y].creature {
                    Some(copy creature) => {
                        actors.push(creature);
                    }
                    None => {}
                }
            }
        }

        // Advance time indefinitely, one loop at a...  time
        loop {
            for actors.each |actor| {
                // Skip actors that no longer exist
                // TODO yeah this sucks  :D
                match actor.location {
                    Nowhere => loop,
                    _ => {},
                }

                while actor.spent_subtics < TIC_SIZE {
                    match actor.act(self, interface) {
                        Some(action) => action.execute(self, interface),
                        None => {}
                    }

                    // Always redraw the world after something happens
                    interface.redraw(self);

                    if self.map.player.health == 0 {
                        fail ~"you died...";
                    }

                    actor.spent_subtics += actor.proto.unspeed;
                }

                // Remove one tic's worth of subtics.  Don't modulo!
                actor.spent_subtics -= TIC_SIZE;
            }

            // Advance the clock
            self.clock += 1;
            // TODO need to re-sort the actor list by time used

            // Always redraw the world at the end of a tic
            interface.redraw(self);
        }
    }
}
