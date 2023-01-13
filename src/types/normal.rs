use crate::types::Double;

#[derive(Copy, Clone, Debug)]
pub struct Normal {
    x: Double,
    y: Double,
    z: Double
}

impl Normal {
    pub fn new(x: Double, y: Double, z: Double) -> Normal {
        Normal {x, y, z}
    }

    pub fn up() -> Normal {
        Normal {
            0.0, 0.0, 1.0
        }
    }

    pub fn normalize(&mut self) {
        let length: Double = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x = self.x / length;
        self.y = self.y / length;
        self.z = self.z / length;
    }
}