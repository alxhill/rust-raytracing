use crate::types::{Double, Point2D};
use crate::world::ViewPlane;
use crate::Point3D;
use rand::prelude::*;
use std::sync::atomic::AtomicU32;

const DEFAULT_NUM_SETS: u32 = 83;

pub trait Sampler {
    fn generate_samples(&mut self);
    fn sample_unit_square(&self) -> Point2D;
    fn num_samples(&self) -> u32;
}

pub struct SamplerInternal {
    pub num_samples: u32,
    pub num_sets: u32,
    pub samples: Vec<Point2D>,
    pub disk_samples: Vec<Point2D>,
    pub hemisphere_samples: Vec<Point3D>,
    index: AtomicU32,
    // shuffled_indices: Vec<u32>,
    // count: u32,
    // jump: u32,
}

impl SamplerInternal {
    pub fn new(num_samples: u32, num_sets: u32) -> SamplerInternal {
        if (num_samples as Double).sqrt().fract() != 0.0 {
            panic!("num_samples must be a perfect square");
        }

        let mut rng = thread_rng();
        SamplerInternal {
            num_samples,
            num_sets,
            samples: Vec::new(),
            disk_samples: Vec::new(),
            hemisphere_samples: Vec::new(),
            index: rng.gen_range(0..(num_samples * num_sets)).into(),
            // shuffled_indices: Vec::new(),
            // count: 0,
            // jump: 0,
        }
        // s.setup_shuffled_indices();
    }

    // fn setup_shuffled_indices(&mut self) {
    //     self.shuffled_indices = (0..self.num_samples)
    //         .collect::<Vec<u32>>()
    //         .repeat(self.num_sets as usize);
    //     let mut rng = thread_rng();
    //     self.shuffled_indices.shuffle(&mut rng);
    // }

    pub fn sample_unit_square(&self) -> Point2D {
        let idx = self
            .index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.samples[(idx + 1 % (self.num_samples * self.num_sets)) as usize]
    }
}

impl Sampler for ViewPlane {
    fn generate_samples(&mut self) {}

    fn sample_unit_square(&self) -> Point2D {
        Point2D::new(0.5, 0.5)
    }

    fn num_samples(&self) -> u32 {
        1
    }
}

pub struct RegularSampler {
    sampler_internal: SamplerInternal,
}

impl RegularSampler {
    pub fn new(samples: u32) -> RegularSampler {
        RegularSampler {
            sampler_internal: SamplerInternal::new(samples, DEFAULT_NUM_SETS),
        }
    }
}

impl Sampler for RegularSampler {
    fn generate_samples(&mut self) {
        let n = (self.sampler_internal.num_samples as Double).sqrt() as u32;

        for i in 0..n {
            for j in 0..n {
                let point = Point2D::new(
                    (i as Double + 0.5) / n as Double,
                    (j as Double + 0.5) / n as Double,
                );
                self.sampler_internal.samples.push(point);
            }
        }
    }

    fn sample_unit_square(&self) -> Point2D {
        self.sampler_internal.sample_unit_square()
    }

    fn num_samples(&self) -> u32 {
        self.sampler_internal.num_samples
    }
}
