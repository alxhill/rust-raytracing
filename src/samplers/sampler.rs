use crate::types::{Double, Point2D};
use crate::world::{ViewPlane, ViewXY};
use rand::prelude::*;

pub trait Sampler {
    fn generate_samples(&self);
    fn num_samples(&self) -> u32;
}

struct SamplerInternal {
    pub num: u32,
    num_sets: u32,
    samples: Vec<Point2D>,
    shuffled_indices: Vec<u32>,
    count: u32,
    jump: u32,
}

impl SamplerInternal {
    fn new(num: u32, num_sets: u32) -> SamplerInternal {
        SamplerInternal {
            num,
            num_sets,
            samples: Vec::new(),
            shuffled_indices: Vec::new(),
            count: 0,
            jump: 0,
        }
    }

    fn setup_shuffled_indices(&mut self) {
        self.shuffled_indices = (0..self.num * self.num_sets).collect();
        let mut rng = thread_rng();
        self.shuffled_indices.shuffle(&mut rng);
    }

    fn sample_unit_square(&mut self) -> Point2D {
        if self.count % self.num == 0 {
            self.jump = (rng().gen::<u32>() % self.num_sets) * self.num;
        }

        self.count += 1;
        self.samples
            [(self.jump + self.shuffled_indices[(self.jump + self.count - 1) as usize]) as usize]
    }
}

impl Sampler for ViewPlane {
    // base implementation of one point at the center of each pixel
    // fn points_for_pixel(&self, xy: &ViewXY) -> Vec<Point2D> {
    //     let (x, y) = (xy.x() as Double, xy.y() as Double);
    //     let (w, h) = (self.width as Double, self.height as Double);
    //     let xw = self.pixel_size * (x - 0.5 * (w - 1.0));
    //     let yw = self.pixel_size * (y - 0.5 * (h - 1.0));
    //
    //     vec![Point2D::new(xw, yw)]
    // }

    fn generate_samples(&mut self) {
        todo!()
    }

    fn num_samples(&self) -> u32 {
        1
    }
}

pub struct RegularSampler {
    view_plane: ViewPlane,
    samples: u32,
}

impl RegularSampler {
    pub fn new(view_plane: ViewPlane, samples: u32) -> RegularSampler {
        RegularSampler {
            view_plane,
            samples,
        }
    }

    fn points_for_pixel(&self, xy: &ViewXY) -> Vec<Point2D> {
        let (x, y) = (xy.x() as Double, xy.y() as Double);
        let (w, h) = (
            self.view_plane.width as Double,
            self.view_plane.height as Double,
        );

        let mut points = Vec::new();
        let n = (self.samples as Double).sqrt();
        for i in 0..n as u32 {
            for j in 0..n as u32 {
                let xw =
                    self.view_plane.pixel_size * (x - 0.5 * (w - 1.0) + (i as Double + 0.5) / n);
                let yw =
                    self.view_plane.pixel_size * (y - 0.5 * (h - 1.0) + (j as Double + 0.5) / n);
                points.push(Point2D::new(xw, yw));
            }
        }

        points
    }
}

impl Sampler for RegularSampler {
    fn generate_samples(&self) {
        todo!()
    }

    fn num_samples(&self) -> u32 {
        self.samples
    }
}

pub struct JitteredSampler {
    view_plane: ViewPlane,
    sampler_internal: SamplerInternal,
}

impl JitteredSampler {
    pub fn new(view_plane: ViewPlane, samples: u32) -> JitteredSampler {
        JitteredSampler {
            view_plane,
            sampler_internal: SamplerInternal::new(samples, 83),
        }
    }
}

impl Sampler for JitteredSampler {
    fn points_for_pixel(&self, xy: &ViewXY) -> Vec<Point2D> {
        let (x, y) = (xy.x() as Double, xy.y() as Double);
        let (w, h) = (
            self.view_plane.width as Double,
            self.view_plane.height as Double,
        );

        let mut points = Vec::new();
        let n = (self.samples as Double).sqrt();
        for i in 0..n as u32 {
            for j in 0..n as u32 {
                let xw = self.view_plane.pixel_size
                    * (x - 0.5 * (w - 1.0) + (i as Double + random::<Double>()) / n);
                let yw = self.view_plane.pixel_size
                    * (y - 0.5 * (h - 1.0) + (j as Double + random::<Double>()) / n);
                points.push(Point2D::new(xw, yw));
            }
        }

        points
    }

    fn generate_samples(&mut self) {
        let n = (self.samples as Double).sqrt() as u32;

        for _ in 0..self.sampler_internal.num_sets {
            for j in 0..n {
                for k in 0..n {
                    let x = (j as Double + random::<Double>()) / n;
                    let y = (k as Double + random::<Double>()) / n;
                    self.sampler_internal.samples.push(Point2D::new(x, y));
                }
            }
        }
    }
}
