use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Div;
use std::ops::DivAssign;

pub struct Vector2 {
    x: f32,
    y: f32
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x: x, y: y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&mut self) {
        let m = self.magnitude();
        if m > 0.0 || m < 0.0 {
            *self /= m;
        }
    }

    pub fn set_magnitude(&mut self, magnitude: f32) {
        self.normalize();
        *self *= magnitude;
    }

    pub fn limit(&mut self, max: f32) {
        if self.magnitude() > max {
            self.set_magnitude(max);
        }
    }

    pub fn rotate(&mut self, angle: f32) {
        let t  = angle.to_radians();
        let cs = t.cos();
        let sn = t.sin();
        let nx = self.x * cs - self.y * sn;
        let ny = self.x * sn + self.y * cs;

        self.x = nx;
        self.y = ny;
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2::new(
            self.x() + other.x(),
            self.y() + other.y()
        )
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Vector2) {
        *self = Vector2::new(
            self.x() + other.x(),
            self.y() + other.y()
        );
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Vector2 {
        Vector2::new(
            self.x() - other.x(),
            self.y() - other.y()
        )
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Vector2) {
        *self = Vector2::new(
            self.x() - other.x(),
            self.y() - other.y()
        );
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, n: f32) -> Vector2 {
        Vector2::new(
            self.x() * n,
            self.y() * n
        )
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, n: f32) {
        *self = Vector2::new(
            self.x() * n,
            self.y() * n
        );
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, n: f32) -> Vector2 {
        Vector2::new(
            self.x() / n,
            self.y() / n
        )
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, n: f32) {
        *self = Vector2::new(
            self.x() / n,
            self.y() / n
        );
    }
}

impl std::fmt::Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}

/*
fn main() {
    let vec = Vector2::new(10.0, 10.0);
    let vec2 = Vector2::new(20.0, 20.0);
    let mut v = vec + vec2;
    v += Vector2::new(10.0, 10.0);

    println!("Hello, world! {}", v.x());
    println!("{}", v);

    let mut rot_vec = Vector2::new(0.0, 1.0);
    rot_vec.rotate(90.0);
    println!("{}", rot_vec);

    println!("{}", v.magnitude());
    v.set_magnitude(0.5);
    println!("{}", v.magnitude());
    v.limit(0.25);
    println!("{}", v.magnitude());
}
*/
