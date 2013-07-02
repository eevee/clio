pub struct Point {
    x: int,
    y: int,
}
impl Point {
}

impl Sub<Point, Offset> for Point {
    fn sub(&self, other: &Point) -> Offset {
        return Offset{
            dx: self.x - other.x,
            dy: self.y - other.y,
        }
    }
}
impl Add<Offset, Point> for Point {
    fn add(&self, other: &Offset) -> Point {
        return Point{
            x: self.x + other.dx,
            y: self.y + other.dy,
        }
    }
}

// -----------------------------------------------------------------------------

pub struct Offset {
    dx: int,
    dy: int,
}
impl Offset {
    pub fn x_dir(&self) -> int {
        if self.dx < 0 { -1 }
        else if self.dx > 0 { 1 }
        else { 0 }
    }
    pub fn y_dir(&self) -> int {
        if self.dy < 0 { -1 }
        else if self.dy > 0 { 1 }
        else { 0 }
    }

    pub fn x_mag(&self) -> uint {
        if self.dx < 0 { -self.dx as uint }
        else if self.dx > 0 { self.dx as uint }
        else { 0 }
    }
    pub fn y_mag(&self) -> uint {
        if self.dy < 0 { -self.dy as uint }
        else if self.dy > 0 { self.dy as uint }
        else { 0 }
    }

    pub fn is_orthogonal(&self) -> bool {
        return self.dx == 0 || self.dy == 0;
    }
    pub fn is_adjacent(&self) -> bool {
        return self.is_orthogonal() && self.taxicab_length() == 1;
    }

    pub fn taxicab_length(&self) -> uint {
        return
            if self.dx < 0 { -self.dx } else { self.dx } as uint +
            if self.dy < 0 { -self.dy } else { self.dy } as uint;
    }
}



// -----------------------------------------------------------------------------

pub struct Size {
    width: uint,
    height: uint,
}
impl Size {
}

// -----------------------------------------------------------------------------

pub struct Rectangle {
    topleft: Point,
    size: Size,
}
impl Rectangle {
    pub fn bottomright(&self) -> Point {
        return Point{
            x: self.topleft.x + self.size.width as int,
            y: self.topleft.y + self.size.height as int,
        };
    }

    pub fn contains(&self, point: &Point) -> bool {
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
