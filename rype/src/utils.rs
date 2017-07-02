pub struct RealPoint {
    pub x: f64,
    pub y: f64,
}

impl RealPoint {
    pub fn mag_sq(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn mag(&self) -> f64 {
        self.mag_sq().sqrt()
    }

    pub fn sub(&self, other: &RealPoint) -> RealPoint {
        RealPoint {x: other.x - self.x, y: other.y - self.y}
    }

    pub fn add(&self, other: &RealPoint) -> RealPoint {
        RealPoint {x: other.x + self.x, y: other.y + self.y}
    }

    pub fn add_inplace(&mut self, other: &RealPoint) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn mul(&self, other: &RealPoint) -> RealPoint {
        RealPoint {x: other.x * self.x, y: other.y * self.y}
    }

    pub fn to_unit(&self) -> RealPoint {
        let mag = self.mag();
        RealPoint {x: self.x / mag, y: self.y / mag}
    }

    pub fn scale(&self, scale: f64) -> RealPoint {
        RealPoint {x: self.x * scale, y: self.y * scale}
    }
}


pub struct IntPoint {
    pub i: i32,
    pub j: i32,
}

impl IntPoint {
    pub fn mag_sq(&self) -> i32 {
        self.i.pow(2) + self.j.pow(2)
    }

    pub fn mag(&self) -> f64 {
        (self.mag_sq() as f64).sqrt()
    }

    pub fn sub(&self, other: &IntPoint) -> IntPoint {
        IntPoint {i: other.i - self.i, j: other.j - self.j}
    }

    pub fn add(&self, other: &IntPoint) -> IntPoint {
        IntPoint {i: other.i + self.i, j: other.j + self.j}
    }

    pub fn add_inplace(&mut self, other: &IntPoint) {
        self.i += other.i;
        self.j += other.j;
    }

    pub fn mul(&self, other: &IntPoint) -> IntPoint {
        IntPoint {i: other.i * self.i, j: other.j * self.j}
    }

    pub fn to_unit(&self) -> RealPoint {
        let mag = self.mag();
        RealPoint {x: self.i as f64 / mag, y: self.j as f64 / mag}
    }
}
