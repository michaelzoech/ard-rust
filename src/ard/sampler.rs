
#[derive(Clone, Debug)]
pub struct Sampler {
    pub samples: Vec<(f64, f64)>
}

impl Sampler {

    pub fn standard_sampler() -> Sampler {
        Sampler {
            samples: vec![(0.5, 0.5)]
        }
    }

    pub fn regular_sampler(samples_per_axis: usize) -> Sampler {
        let mut samples = Vec::with_capacity(samples_per_axis * samples_per_axis);
        let axis_samples = samples_per_axis as f64;
        let offset = 1.0 / (axis_samples * 2.0);
        for y in 0..samples_per_axis {
            let fy = offset + (y as f64) / axis_samples;
            for x in 0..samples_per_axis {
                let fx = offset + (x as f64) / axis_samples;
                samples.push((fx, fy));
            }
        }
        Sampler {
            samples: samples
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_sampler_one_per_axis() {
        let one_per_axis =  Sampler::regular_sampler(1);
        assert_eq!(1, one_per_axis.samples.len());
        assert_close!(0.5, one_per_axis.samples[0].0);
        assert_close!(0.5, one_per_axis.samples[0].1);
    }

    #[test]
    fn regular_sampler_two_per_axis() {
        let one_per_axis =  Sampler::regular_sampler(2);
        assert_eq!(4, one_per_axis.samples.len());
        assert_close!(0.25, one_per_axis.samples[0].0);
        assert_close!(0.25, one_per_axis.samples[0].1);
        assert_close!(0.75, one_per_axis.samples[1].0);
        assert_close!(0.25, one_per_axis.samples[1].1);
    }
}