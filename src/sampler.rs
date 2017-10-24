use std::f64::consts::{PI};
use rand::{self, Rng};
use rand::distributions::{IndependentSample, Range};

use math::{Vector2, Vector3};

pub trait Sampler<T> where T: Copy + Clone {

    fn sample(&self, set_index: usize, sample_index: usize) -> T;
}

#[derive(Clone, Debug)]
pub struct UnitSquareSampler {
    pub samples: Vec<Vec<Vector2>>,
}

impl Sampler<Vector2> for UnitSquareSampler {

    fn sample(&self, set_index: usize, sample_index: usize) -> Vector2 {
        let mod_set_index = self.samples.len() % set_index;
        let mod_sample_index = self.samples[mod_set_index].len() % sample_index;
        self.samples[mod_set_index][mod_sample_index]
    }
}

impl UnitSquareSampler {

    pub fn standard_sampler() -> UnitSquareSampler {
        UnitSquareSampler {
            samples: vec![vec![Vector2::new(0.5, 0.5)]],
        }
    }

    pub fn regular_sampler(samples_per_axis: usize) -> UnitSquareSampler {
        let mut samples = Vec::with_capacity(samples_per_axis * samples_per_axis);
        let axis_samples = samples_per_axis as f64;
        let offset = 1.0 / (axis_samples * 2.0);

        for y in 0..samples_per_axis {
            let fy = offset + (y as f64) / axis_samples;
            for x in 0..samples_per_axis {
                let fx = offset + (x as f64) / axis_samples;
                samples.push(Vector2::new(fx, fy));
            }
        }

        UnitSquareSampler {
            samples: create_shuffled_samples(&samples, 83),
        }
    }

    pub fn jittered_sampler(samples_per_axis: usize) -> UnitSquareSampler {
        let mut samples = Vec::with_capacity(samples_per_axis * samples_per_axis);
        let mut rng = rand::thread_rng();
        let between = Range::new(0.0f64, 1.0);
        let box_dim = 1.0 / (samples_per_axis as f64);

        for y in 0..samples_per_axis {
            for x in 0..samples_per_axis {
                let fx = box_dim * (x as f64) + box_dim * between.ind_sample(&mut rng);
                let fy = box_dim * (y as f64) + box_dim * between.ind_sample(&mut rng);
                samples.push(Vector2::new(fx, fy));
            }
        }

        UnitSquareSampler {
            samples: create_shuffled_samples(&samples, 83),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HemiSphereSampler {
    pub samples: Vec<Vec<Vector3>>,
}

impl Sampler<Vector3> for HemiSphereSampler {

    fn sample(&self, set_index: usize, sample_index: usize) -> Vector3 {
        let mod_set_index = set_index % self.samples.len();
        let mod_sample_index = sample_index % self.samples[mod_set_index].len();
        self.samples[mod_set_index][mod_sample_index]
    }
}

impl HemiSphereSampler {

    pub fn standard_sampler() -> HemiSphereSampler {
        HemiSphereSampler {
            samples: vec![vec![Vector3::new(0.0, 0.0, 1.0)]],
        }
    }

    pub fn regular_sampler(samples_per_axis: usize, e: f64) -> HemiSphereSampler {
        from_unit_square_sampler(&UnitSquareSampler::regular_sampler(samples_per_axis), e)
    }

    pub fn jittered_sampler(samples_per_axis: usize, e: f64) -> HemiSphereSampler {
        from_unit_square_sampler(&UnitSquareSampler::jittered_sampler(samples_per_axis), e)
    }
}

fn from_unit_square_sampler(sampler: &UnitSquareSampler, e: f64) -> HemiSphereSampler {
    HemiSphereSampler {
        samples: sampler.samples.iter().map(|v| {
            v.iter().map(|p| unit_square_sample_to_hemisphere_sample(e, *p)).collect()
        }).collect(),
    }
}

#[derive(Clone, Debug)]
pub struct UnitSphereSampler {
    pub samples: Vec<Vec<Vector3>>,
}

impl Sampler<Vector3> for UnitSphereSampler {

    fn sample(&self, set_index: usize, sample_index: usize) -> Vector3 {
        let mod_set_index = set_index % self.samples.len();
        let mod_sample_index = sample_index % self.samples[mod_set_index].len();
        self.samples[mod_set_index][mod_sample_index]
    }
}

impl UnitSphereSampler {

    pub fn standard_sampler() -> UnitSphereSampler {
        UnitSphereSampler {
            samples: vec![vec![Vector3::new(0.0, 0.0, 0.0)]],
        }
    }

    pub fn random_sampler(num_samples: usize) -> UnitSphereSampler {
        let mut samples = Vec::with_capacity(num_samples);
        let mut rng = rand::thread_rng();

        while samples.len() < num_samples {
            let v = Vector3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0));
            if v.length_squared() > 1.0 {
                continue
            }
            samples.push(v);
        }

        UnitSphereSampler {
            samples: create_shuffled_samples(&samples, 83),
        }
    }
}

fn unit_square_sample_to_hemisphere_sample(e: f64, sample: Vector2) -> Vector3 {
    let cosphi = (2.0 * PI * sample.x).cos();
    let sinphi = (2.0 * PI * sample.x).sin();
    let costheta = (1.0 - sample.y).powf( 1.0 / (e + 1.0));
    let sintheta = (1.0 - costheta * costheta).sqrt();

    Vector3 {
        x: sintheta * cosphi,
        y: sintheta * sinphi,
        z: costheta,
    }
}

fn create_shuffled_samples<T>(samples: &Vec<T>, num_sets: usize) -> Vec<Vec<T>> where T: Copy {
    let mut rng = rand::thread_rng();
    let mut sets: Vec<Vec<T>> = Vec::with_capacity(num_sets);
    let num_samples = samples.len();

    for i in 0..num_sets {
        let mut indices: Vec<usize> = (0..num_samples).collect();
        rng.shuffle(&mut indices.as_mut_slice());
        sets.push(Vec::with_capacity(num_samples));
        for k in 0..num_samples {
            sets[i].push(samples[indices[k]]);
        }
    }

    sets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_sampler_one_per_axis() {
        let one_per_axis =  UnitSquareSampler::regular_sampler(1);
        assert_eq!(true, one_per_axis.samples.len() > 0);
        assert_eq!(1, one_per_axis.samples[0].len());
        assert_close!(0.5, one_per_axis.samples[0][0].x);
        assert_close!(0.5, one_per_axis.samples[0][0].y);
    }
}