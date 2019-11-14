use std::ops::Add;

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn up() -> Self {
        Point::new(0, -1)
    }

    pub fn down() -> Self {
        Point::new(0, 1)
    }

    pub fn left() -> Self {
        Point::new(-1, 0)
    }

    pub fn right() -> Self {
        Point::new(1, 0)
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub p1: Point,
    pub p2: Point,
}

impl Rect {
    pub fn new(p1: Point, w: i32, h: i32) -> Self {
        Rect {
            p1,
            p2: Point {
                x: p1.x + w,
                y: p1.y + h,
            },
        }
    }
}
