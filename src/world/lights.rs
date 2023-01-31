use crate::types::{Double, Point3D, RGBColor, Vector3D};

#[derive(Debug, Copy, Clone)]
pub struct Light {
    color: RGBColor,
    location: Point3D,
    shadows: bool,
    ls: Double // may become a material in future
}

impl Light {
    pub fn point_light(point: Point3D, ls: Double) -> Light {
        Light {
            color: RGBColor::WHITE,
            location: point,
            shadows: false,
            ls
        }
    }

    pub fn direction(&self, hit_point: &Point3D) -> Vector3D {
        (self.location - *hit_point).normalize()
    }

    pub fn l(&self) -> RGBColor {
        self.color * self.ls
    }
}