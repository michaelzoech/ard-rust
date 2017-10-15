use std::f64::consts::{PI};
use rand;
use rand::distributions::{IndependentSample, Range};

use math::{Vector3};

#[derive(Clone, Debug)]
pub struct Sampler {
    pub unit_square_samples: Vec<(f64, f64)>,
    pub hemisphere_samples: Vec<Vector3>,
}

impl Sampler {

    pub fn standard_sampler() -> Sampler {
        let unit_square_samples = vec![(0.5, 0.5)];
        let hemisphere_samples = unit_square_samples.iter().map(|p| unit_square_sample_to_hemisphere_sample(10.0, *p)).collect();
        Sampler {
            unit_square_samples: unit_square_samples,
            hemisphere_samples: hemisphere_samples,
        }
    }

    pub fn regular_sampler(samples_per_axis: usize, e: f64) -> Sampler {
        let mut unit_square_samples = Vec::with_capacity(samples_per_axis * samples_per_axis);
        let axis_samples = samples_per_axis as f64;
        let offset = 1.0 / (axis_samples * 2.0);
        for y in 0..samples_per_axis {
            let fy = offset + (y as f64) / axis_samples;
            for x in 0..samples_per_axis {
                let fx = offset + (x as f64) / axis_samples;
                unit_square_samples.push((fx, fy));
            }
        }
        let hemisphere_samples = unit_square_samples.iter().map(|p| unit_square_sample_to_hemisphere_sample(e, *p)).collect();
        Sampler {
            unit_square_samples: unit_square_samples,
            hemisphere_samples: hemisphere_samples,
        }
    }

    pub fn jittered_sampler(samples_per_axis: usize, e: f64) -> Sampler {
        let mut unit_square_samples = Vec::with_capacity(samples_per_axis * samples_per_axis);

        let mut rng = rand::thread_rng();
        let between = Range::new(0.0f64, 1.0);
        let box_dim = 1.0 / (samples_per_axis as f64);

        for y in 0..samples_per_axis {
            for x in 0..samples_per_axis {
                let fx = box_dim * (x as f64) + box_dim * between.ind_sample(&mut rng);
                let fy = box_dim * (y as f64) + box_dim * between.ind_sample(&mut rng);
                unit_square_samples.push((fx, fy));
            }
        }
        let hemisphere_samples = unit_square_samples.iter().map(|p| unit_square_sample_to_hemisphere_sample(e, *p)).collect();
        Sampler {
            unit_square_samples: unit_square_samples,
            hemisphere_samples: hemisphere_samples,
        }
    }
}

fn unit_square_sample_to_hemisphere_sample(e: f64, point: (f64, f64)) -> Vector3 {
    let cosphi = (2.0 * PI * point.0).cos();
    let sinphi = (2.0 * PI * point.0).sin();
    let costheta = (1.0 - point.1).powf( 1.0 / (e + 1.0));
    let sintheta = (1.0 - costheta * costheta).sqrt();

    Vector3 {
        x: sintheta * cosphi,
        y: sintheta * sinphi,
        z: costheta,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_sampler_one_per_axis() {
        let one_per_axis =  Sampler::regular_sampler(1, 0.0);
        assert_eq!(1, one_per_axis.samples.len());
        assert_close!(0.5, one_per_axis.samples[0].0);
        assert_close!(0.5, one_per_axis.samples[0].1);
    }

    #[test]
    fn regular_sampler_two_per_axis() {
        let one_per_axis =  Sampler::regular_sampler(2, 0.0);
        assert_eq!(4, one_per_axis.samples.len());
        assert_close!(0.25, one_per_axis.samples[0].0);
        assert_close!(0.25, one_per_axis.samples[0].1);
        assert_close!(0.75, one_per_axis.samples[1].0);
        assert_close!(0.25, one_per_axis.samples[1].1);
    }
}