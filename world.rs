use std::managed;
use std::uint;
use std::util::swap;

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
    grid: ~[~[@mut Tile]],
    player: @mut Entity,
}
impl Map {
    pub fn width(&self) -> uint {
        return self.size.width;
    }
    pub fn height(&self) -> uint {
        return self.size.height;
    }
    fn bounds(&self) -> Rectangle {
        return Rectangle{
            topleft: Point{ x: 0, y: 0 },
            size: self.size,
        };
    }

    pub fn player_tile(&self) -> @mut Tile {
        match self.player.location {
            OnFloor(point) => {
                return self.grid[point.x][point.y];
            }
            _ => fail!(~"todo"),
        }
    }
    pub fn tile_relative(&self, source: @mut Entity, offset: Offset) -> Option<@mut Tile> {
        let point = match source.location {
            OnFloor(pt) => pt,
            _ => fail!(~"todo"),
        };
        let new_point = point + offset;

        if self.bounds().contains(&new_point) {
            return Some(self.grid[new_point.x][new_point.y])
        }
        else {
            return None;
        }
    }

    pub fn remove_entity(&mut self, entity: @mut Entity) {
        match entity.location {
            OnFloor(point) => {
                let tile = self.grid[point.x][point.y];
                match tile.creature {
                    Some(creature) if managed::mut_ptr_eq(entity, creature) => {
                        entity.location = Nowhere;
                        tile.creature = None;
                    }
                    _ => {
                        fail!(~"Entity not where it claimed to be!");
                    }
                }
            }
            _ => {
                fail!(~"Don't know how to remove this entity");
            }
        }
    }

    pub fn move_entity(&mut self, entity: @mut Entity, dx: int, dy: int) {
        match entity.location {
            OnFloor(point) => {
                let new_x = point.x + dx;
                let new_y = point.y + dy;
                // TODO point type?
                // TODO check in bounds...
                let target_tile = self.grid[new_x][new_y];
                // TODO these checks should already be done by the time we get here
                if (target_tile.architecture).is_passable() && target_tile.creature.is_none() {
                    self.grid[point.x][point.y].creature = None;
                    entity.location = OnFloor(Point{ x: new_x, y: new_y });
                    target_tile.creature = Some(entity);
                }
            }
            _ => fail!(~"Can't move an entity that's not on the dungeon floor"),
        }
    }
}

pub struct Tile {
    architecture: @mut Entity,
    creature: Option<@mut Entity>,
    items: ~[@mut Entity],
}


/// Number of subtics (speed units) per clock tick
static TIC_SIZE: uint = 48;

pub struct World {
    map: @mut Map,
    clock: uint,
}
pub fn new_game() -> @mut World {
    return @mut World{ map: generate_map(), clock: 0 };
}
impl World {
    /** Runs the game forever.  Ish. */
    pub fn run(@mut self, interface: @Interface) {
        // TODO this should really take a & but i keep tripping over
        // https://github.com/mozilla/rust/issues/5708

        // Draw the game world first
        interface.redraw(self);

        // Find everything in the world.
        // TODO extend this to letting every object in the world advance by one
        // clock tic; make it that generic componenty entry point of update()
        // (PS: that includes recursing into containers
        // TODO this will fuck up if objects are created or destroyed, the map
        // changes, etc.!  should possibly be an attribute of the map, so it
        // can be responsible for maintaining the order

        let mut actors: ~[@mut Entity] = ~[];
        for uint::range(0, self.map.width()) |x| {
            for uint::range(0, self.map.height()) |y| {
                match self.map.grid[x][y].creature {
                    Some(creature) => {
                        actors.push(creature);
                    }
                    None => {}
                }
            }
        }

        // Advance time indefinitely, one loop at a...  time
        loop {
            for actors.iter().advance |actor| {
                // Skip actors that no longer exist
                // TODO yeah this sucks  :D
                match actor.location {
                    Nowhere => loop,
                    _ => {},
                }

                while actor.spent_subtics < TIC_SIZE {
                    match actor.act(self, &interface) {
                        Some(action) => action.execute(self, &interface),
                        None => {}
                    }

                    // Always redraw the world after something happens
                    interface.redraw(self);

                    if self.map.player.health == 0 {
                        fail!(~"you died...");
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
