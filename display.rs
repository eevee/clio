use world::Map;

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
