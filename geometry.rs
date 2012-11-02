struct Point {
    x: uint,
    y: uint,
}
impl Point {
}

impl Point: Sub<Point, Offset> {
    pure fn sub(other: &Point) -> Offset {
        return Offset{
            dx: self.x as int - other.x as int,
            dy: self.y as int - other.y as int,
        }
    }
}

// -----------------------------------------------------------------------------

struct Offset {
    dx: int,
    dy: int,
}
impl Offset {
    fn x_dir() -> int {
        if self.dx < 0 { -1 }
        else if self.dx > 0 { 1 }
        else { 0 }
    }
    fn y_dir() -> int {
        if self.dy < 0 { -1 }
        else if self.dy > 0 { 1 }
        else { 0 }
    }

    fn x_mag() -> uint {
        if self.dx < 0 { -self.dx as uint }
        else if self.dx > 0 { self.dx as uint }
        else { 0 }
    }
    fn y_mag() -> uint {
        if self.dy < 0 { -self.dy as uint }
        else if self.dy > 0 { self.dy as uint }
        else { 0 }
    }

    fn is_orthogonal() -> bool {
        return self.dx == 0 || self.dy == 0;
    }
    fn is_adjacent() -> bool {
        return self.is_orthogonal() && self.taxicab_length() == 1;
    }

    fn taxicab_length() -> uint {
        return
            if self.dx < 0 { -self.dx } else { self.dx } as uint +
            if self.dy < 0 { -self.dy } else { self.dy } as uint;
    }
}



// -----------------------------------------------------------------------------

struct Size {
    width: uint,
    height: uint,
}
impl Size {
}

// -----------------------------------------------------------------------------

struct Rectangle {
    topleft: Point,
    size: Size,
}
impl Rectangle {
}
