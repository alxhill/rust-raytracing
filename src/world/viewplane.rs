use crate::types::Double;

#[derive(Debug, Copy, Clone)]
pub struct ViewPlane {
    pub hres: u32,
    pub vres: u32,
    pub s: Double,
    pub gamma: Double,
    pub inv_gamma: Double,
    pub show_out_of_gamut: bool,
}

impl ViewPlane {
    pub fn new(hres: u32, vres: u32, s: Double) -> ViewPlane {
        ViewPlane {
            hres,
            vres,
            s,
            gamma: 1.0,
            inv_gamma: 1.0,
            show_out_of_gamut: false,
        }
    }
}
