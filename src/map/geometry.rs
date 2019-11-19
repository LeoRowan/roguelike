use std::ops::{Add, Div};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
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

    pub fn distance_to(self, other: Point) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Div<i32> for Point {
    type Output = Point;

    fn div(self, rhs: i32) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs)
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

    pub fn center(&self) -> Point {
        (self.p1 + self.p2) / 2
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        self.p1.x <= other.p2.x
            && self.p2.x >= other.p1.x
            && self.p1.y <= other.p2.y
            && self.p2.y >= other.p1.y
    }
}
