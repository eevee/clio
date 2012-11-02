use amulet::ll;
use amulet::ll::Style;

// TODO should probably distinguish somehow between architecture, creatures,
// and flooring in a static way
const NORMAL: Style = Style{ is_bold: false, is_underline: false, fg_color: -1, bg_color: -1 };
pub const ROCKFACE: Prototype = Prototype{ display: ' ', style: Style{ is_bold: false, is_underline: false, fg_color: -1, bg_color: -1 }, passable: false };
pub const WALL:     Prototype = Prototype{ display: '▒', style: Style{ is_bold: false, is_underline: false, fg_color: 8, bg_color: -1 }, passable: false };
pub const FLOOR:    Prototype = Prototype{ display: '·', style: Style{ is_bold: false, is_underline: false, fg_color: 8, bg_color: -1 }, passable: true };
pub const PLAYER:   Prototype = Prototype{ display: '☻', style: Style{ is_bold: false, is_underline: false, fg_color: 4, bg_color: -1 }, passable: false };
pub const SCROLL:   Prototype = Prototype{ display: '?', style: Style{ is_bold: true, is_underline: false, fg_color: -1, bg_color: -1 }, passable: true };
pub const ENEMY:    Prototype = Prototype{ display: 'a', style: Style{ is_bold: true, is_underline: false, fg_color: 1, bg_color: -1 }, passable: true };


pub enum Location {
    Nowhere,
    OnFloor(uint, uint),
    InContainer,
}

struct Entity {
    proto: &static/Prototype,
    mut location: Location,
    mut contents: ~[@Entity],
}
impl Entity {
    // PHYSICS
    fn is_passable() -> bool {
        return self.proto.passable;
    }
}


struct Prototype {
    display: char,
    style: ll::Style,
    passable: bool,
}
impl &static/Prototype {
    fn make_entity() -> @Entity {
        return @Entity{
            proto: self,
            location: Nowhere,
            contents: ~[],
        };
    }
}
