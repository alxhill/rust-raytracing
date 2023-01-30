use crate::types::{Double, Point3D, RGBColor, Vector3D};

#[derive(Debug, Copy, Clone)]
pub struct AmbientLight {
    color: RGBColor,
    ls: Double
}

impl AmbientLight{
    pub fn default() -> AmbientLight {
        AmbientLight {
            color: RGBColor::WHITE,
            ls: 1.0
        }
    }

    pub fn L(&self) -> RGBColor {
        self.color * self.ls
    }
}


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

    pub fn L(&self) -> RGBColor {
        self.color * self.ls
    }
}