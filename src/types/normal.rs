use crate::types::Double;

#[derive(Copy, Clone, Debug)]
pub struct Normal {
    x: Double,
    y: Double,
    z: Double
}

impl Normal {
    fn new(x: Double, y: Double, z: Double) -> Normal {
        Normal {x, y, z}
    }

    fn normalize(&mut self) {
        let length: Double = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x = self.x / length;
        self.y = self.y / length;
        self.z = self.z / length;
    }
}