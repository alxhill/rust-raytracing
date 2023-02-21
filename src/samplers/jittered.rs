use rand::prelude::*;
use crate::{Double, Sampler};
use crate::types::Point2D;

pub struct JitteredSampler {
    sampler_internal: SamplerInternal,
}

impl JitteredSampler {
    pub fn new(samples: u32) -> JitteredSampler {
        let mut j = JitteredSampler {
            sampler_internal: SamplerInternal::new(samples, 83),
        };
        j.generate_samples();
        j
    }
}

impl Sampler for JitteredSampler {
    fn generate_samples(&mut self) {
        let n = (self.sampler_internal.num_samples as Double).sqrt() as u32;

        for _ in 0..self.sampler_internal.num_sets {
            for j in 0..n {
                for k in 0..n {
                    let x = (j as Double + random::<Double>()) / n as Double;
                    let y = (k as Double + random::<Double>()) / n as Double;
                    self.sampler_internal.samples.push(Point2D::new(x, y));
                }
            }
        }
    }

    fn sample_unit_square(&mut self) -> Point2D {
        self.sampler_internal.sample_unit_square()
    }

    fn num_samples(&self) -> u32 {
        self.sampler_internal.num_samples()
    }
}
