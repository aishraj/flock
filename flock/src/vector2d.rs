use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Vector2D {
        Vector2D { x, y }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vector2D {
        let length = self.length();
        Vector2D::new(self.x / length, self.y / length)
    }

    pub fn dot(&self, other: &Vector2D) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Vector2D) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn angle(&self, other: &Vector2D) -> f64 {
        let dot = self.dot(other);
        let cross = self.cross(other);
        cross.atan2(dot)
    }

    pub fn rotate(&self, angle: f64) -> Vector2D {
        let x = self.x * angle.cos() - self.y * angle.sin();
        let y = self.x * angle.sin() + self.y * angle.cos();
        Vector2D::new(x, y)
    }

    pub fn add(&self, other: &Vector2D) -> Vector2D {
        Vector2D::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(&self, other: &Vector2D) -> Vector2D {
        Vector2D::new(self.x - other.x, self.y - other.y)
    }

    pub fn mul(&self, other: &Vector2D) -> Vector2D {
        Vector2D::new(self.x * other.x, self.y * other.y)
    }

    pub fn div(&self, other: &Vector2D) -> Vector2D {
        Vector2D::new(self.x / other.x, self.y / other.y)
    }

    pub fn scale(&self, scale: f64) -> Vector2D {
        Vector2D::new(self.x * scale, self.y * scale)
    }

    pub fn distance(&self, other: &Vector2D) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        (x * x + y * y).sqrt()
    }

    pub fn limit(&self, max: f64) -> Vector2D {
        let length = self.length();
        if length > max {
            self.scale(max / length)
        } else {
            *self
        }
    }
}

impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}           
