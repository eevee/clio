use entity::Entity;

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
        let (x, y) = self.player.position;
        return self.grid[x][y];
    }

    fn move_player(dx: int, dy: int) {
        let (x, y) = self.player.position;
        let new_x = (x as int + dx) as uint;
        let new_y = (y as int + dy) as uint;
        // TODO point type?
        // TODO check in bounds...
        let target_tile = self.grid[new_x][new_y];
        if target_tile.architecture.is_passable() && target_tile.creature.is_none() {
            self.player.position = (new_x, new_y);
            self.grid[x][y].creature = None;
            self.grid[new_x][new_y].creature = Some(self.player);
        }
    }
}

struct Tile {
    mut architecture: @Entity,
    mut creature: Option<@Entity>,
    mut items: ~[@Entity],
}


