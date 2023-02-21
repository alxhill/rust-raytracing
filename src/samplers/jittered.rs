use crate::samplers::{Sampler, SamplerInternal};
use crate::types::{Double, Point2D};
use rand::prelude::*;

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
        self.sampler_internal.num_samples
    }
}

pub struct MultiJittered {
    sampler_internal: SamplerInternal,
}

impl MultiJittered {
    pub fn new(samples: u32) -> MultiJittered {
        let mut j = MultiJittered {
            sampler_internal: SamplerInternal::new(samples, 83),
        };
        j.generate_samples();
        j
    }
}

impl Sampler for MultiJittered {
    fn generate_samples(&mut self) {
        let mut rng = thread_rng();
        let s = self.sampler_internal.num_samples as Double;
        let n = s.sqrt() as u32;
        let subcell_width = 1.0 / s;

        self.sampler_internal
            .samples
            .resize((n * n) as usize, Point2D::new(0.0, 0.0));

        for p in 0..self.sampler_internal.num_sets {
            for i in 0..n {
                for j in 0..n {
                    let np = Point2D::new(i as Double, j as Double);
                    let sp =
                        (np * n + np.swap()) * subcell_width + rng.gen_range(0.0..subcell_width);
                    self.sampler_internal.samples
                        [(i * n + j + p * self.sampler_internal.num_samples) as usize] = sp;
                }
            }
        }

        for p in 0..self.sampler_internal.num_sets {
            for i in 0..n {
                for j in 0..n {
                    let k = rng.gen_range(j..n) as usize;
                    let l = rng.gen_range(j..n) as usize;

                    let s = &mut self.sampler_internal.samples;

                    let base_index = (i * n + p * self.sampler_internal.num_samples) as usize;
                    let ju = j as usize;

                    // get a random sample from the set
                    let tmp = s[base_index + ju];

                    // set the random sample's x value to another random sample's x value
                    s[base_index + ju].x = s[base_index + k].x;
                    s[base_index + k].x = tmp.x;

                    // set the random sample's y value to another random sample's y value
                    s[base_index + ju].y = s[base_index + l].y;
                    s[base_index + l].y = tmp.y;
                }
            }
        }
    }

    fn sample_unit_square(&mut self) -> Point2D {
        self.sampler_internal.sample_unit_square()
    }

    fn num_samples(&self) -> u32 {
        self.sampler_internal.num_samples
    }
}
