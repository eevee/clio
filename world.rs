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

    fn move_player(dx: int, dy: int) {
        match self.player.location {
            OnFloor(copy x, copy y) => {
                let new_x = (x as int + dx) as uint;
                let new_y = (y as int + dy) as uint;
                // TODO point type?
                // TODO check in bounds...
                let target_tile = self.grid[new_x][new_y];
                if target_tile.architecture.is_passable() && target_tile.creature.is_none() {
                    self.player.location = OnFloor(new_x, new_y);
                    self.grid[x][y].creature = None;
                    self.grid[new_x][new_y].creature = Some(self.player);
                }
            }
            _ => fail,
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
