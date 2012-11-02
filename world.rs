use display::TerminalDisplay;
use entity::Entity;
use entity::OnFloor;
use fractor::generate_map;

struct Map {
    size: (uint, uint),
    mut grid: ~[~[@Tile]],
    mut player: @Entity,

    // TODO this goes on the world, really.
    mut clock: uint,
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
        match self.player.location {
            OnFloor(copy x, copy y) => {
                return self.grid[x][y];
            }
            _ => fail,
        }
    }

    fn move_entity(entity: @Entity, dx: int, dy: int) {
        match entity.location {
            OnFloor(copy x, copy y) => {
                let new_x = (x as int + dx) as uint;
                let new_y = (y as int + dy) as uint;
                // TODO point type?
                // TODO check in bounds...
                let target_tile = self.grid[new_x][new_y];
                // TODO these checks should already be done by the time we get here
                if (copy target_tile.architecture).is_passable() && target_tile.creature.is_none() {
                    entity.location = OnFloor(new_x, new_y);
                    self.grid[new_x][new_y].creature <-> self.grid[x][y].creature;
                }
            }
            _ => fail ~"Can't move an entity that's not on the dungeon floor",
        }
    }
}

struct Tile {
    mut architecture: @Entity,
    mut creature: Option<@Entity>,
    mut items: ~[@Entity],
}


struct Game {
    map: @Map,
}
pub fn new_game() -> @Game {
    return @Game{ map: generate_map() };
}
impl Game {
    /** Runs the game until player input is required. */
    fn advance_clock(@self, display: &TerminalDisplay) {
        // TODO extend this to letting every object in the world advance by one
        // clock tic; make it that generic componenty entry point of update()
        // (PS: that includes recursing into containers
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

        for actors.each |actor| {
            match actor.act(self) {
                Some(action) => action.execute(self, display),
                None => {}
            }

            if self.map.player.health == 0 {
                fail ~"you died...";
            }
        }
    }
}
