extern mod amulet;

fn main() {
    let mut map = vec::from_fn(10, |i| {
        vec::from_fn(10, |j| {
            if i == 0 || i == 9 || j == 0 || j == 9 {
                Tile{ object: option::Some(~WALL) }
            }
            else {
                Tile{ object: option::Some(~FLOOR) }
            }
        })
    });

    map[3][3] = Tile{ object: option::Some(~PLAYER) };

    let window = amulet::ll::init_screen();

    for map.eachi |i, row| {
        for row.eachi |j, tile| {
            window.mv(i, j);
            window.print(fmt!("%c", tile.object.get().display));
        }
    }

    window.repaint();
    window.getch();
}

////////////////////////////////////////////////////////////////////////////////

struct Tile {
    object: option::Option<~Prototype>,
}



struct Prototype {
    display: char,
    passable: bool,
}


const WALL: Prototype = Prototype{ display: '#', passable: false };
const FLOOR: Prototype = Prototype{ display: ' ', passable: true };
const PLAYER: Prototype = Prototype{ display: '@', passable: false };

