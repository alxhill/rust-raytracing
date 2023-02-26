use crate::types::{Double, Point2D};
use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

type X = u32;
type Y = u32;

#[derive(Debug, Copy, Clone)]
pub struct ViewXY(pub X, pub Y);

impl ViewXY {
    pub fn x(&self) -> X {
        self.0
    }
    pub fn y(&self) -> Y {
        self.1
    }
}

impl From<&ViewXY> for Point2D {
    fn from(value: &ViewXY) -> Self {
        Point2D {
            x: value.x() as Double,
            y: value.y() as Double,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Screen {
    pub width: Double,
    pub height: Double,
    pub pixel_size: Double,
}

#[derive(Debug, Copy, Clone)]
pub struct ViewPlane {
    pub width: X,
    pub height: Y,
    pub pixel_size: Double,
    pub gamma: Double,
    pub inv_gamma: Double,
    pub show_out_of_gamut: bool,
}

impl ViewPlane {
    pub fn new(width: u32, height: u32, pixel_size: Double) -> ViewPlane {
        ViewPlane {
            width,
            height,
            pixel_size,
            gamma: 1.0,
            inv_gamma: 1.0,
            show_out_of_gamut: false,
        }
    }

    pub fn for_each_pixel(&self, mut f: impl FnMut(&ViewXY)) {
        for x in 0..self.width {
            for y in 0..self.height {
                f(&ViewXY(x, y));
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ViewPlaneIter {
    view_plane: ViewPlane,
    x: X,
    y: Y,
}

impl IntoIterator for &ViewPlane
where
    ViewPlane: Send + Sync,
{
    type Item = ViewXY;
    type IntoIter = ViewPlaneIter;

    fn into_iter(self) -> Self::IntoIter {
        ViewPlaneIter {
            view_plane: *self,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for ViewPlaneIter {
    type Item = ViewXY;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.view_plane.height {
            return None;
        }
        let result = ViewXY(self.x, self.y);
        self.x += 1;
        if self.x >= self.view_plane.width {
            self.x = 0;
            self.y += 1;
        }
        Some(result)
    }
}

#[cfg(feature = "parallel")]
impl IntoParallelIterator for ViewPlane {
    type Iter = ViewPlaneIter;
    type Item = ViewXY;

    fn into_par_iter(self) -> Self::Iter {
        ViewPlaneIter {
            view_plane: self,
            x: 0,
            y: 0,
        }
    }
}

#[cfg(feature = "parallel")]
impl ParallelIterator for ViewPlaneIter {
    type Item = ViewXY;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        let (x, y) = (self.x, self.y);
        let (width, height) = (self.view_plane.width, self.view_plane.height);
        (y..height)
            .into_par_iter()
            .flat_map(move |y| (x..width).into_par_iter().map(move |x| ViewXY(x, y)))
            .drive_unindexed(consumer)
    }
}
