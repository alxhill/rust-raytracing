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
    direction: Vector3D,
    location: Point3D,
    shadows: bool,
    ls: Double // may become a material in future
}

impl Light {
    pub fn point_at(point: Point3D, look_at: Point3D, ls: Double) -> Light {
        Light {
            color: RGBColor::WHITE,
            direction: look_at - point,
            location: point,
            shadows: false,
            ls
        }
    }

    pub fn direction(&self) -> Vector3D {
        self.direction
    }

    pub fn L(&self) -> RGBColor {
        self.color * self.ls
    }
}