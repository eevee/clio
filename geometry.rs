struct Point {
    x: int,
    y: int,
}
impl Point {
}

impl Point: Sub<Point, Offset> {
    pure fn sub(other: &Point) -> Offset {
        return Offset{
            dx: self.x - other.x,
            dy: self.y - other.y,
        }
    }
}
impl Point: Add<Offset, Point> {
    pure fn add(other: &Offset) -> Point {
        return Point{
            x: self.x + other.dx,
            y: self.y + other.dy,
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
    fn bottomright() -> Point {
        return Point{
            x: self.topleft.x + self.size.width as int,
            y: self.topleft.y + self.size.height as int,
        };
    }

    fn contains(point: &Point) -> bool {
        if point.x < self.topleft.x || point.y < self.topleft.y {
            return false;
        }

        let bottomright = self.bottomright();
        if point.x > bottomright.x || point.y > bottomright.y {
            return false;
        }

        return true;
    }
}
