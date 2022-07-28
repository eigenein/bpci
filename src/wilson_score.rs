use num_traits::{Float, One, PrimInt, Unsigned};

use crate::{Interval, Sample};

impl<N, F> Sample<N, F>
where
    F: Float + One,
    N: PrimInt + Unsigned + Into<F>,
{
    /// [Wilson score interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Wilson_score_interval)
    pub fn wilson_score_interval(&self, z_level: F) -> Interval<F> {
        let sample_size = self.size.into();
        let p_hat = self.p_hat();

        let one = F::one();
        let two = F::one() + F::one();
        let four = two + two;

        let a = z_level * z_level / sample_size;
        let b = one / (one + a);

        let mean = b * (p_hat + a / two);
        let margin =
            z_level * b * (p_hat * (one - p_hat) / sample_size + a / sample_size / four).sqrt();

        Interval { mean, margin }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;
    use crate::Proportion;

    #[test]
    fn wilson_score_ok() {
        let sample = Sample::new(20, Proportion::NSuccesses(10_u32)).unwrap();
        let interval = sample.wilson_score_interval(1.960);
        assert_relative_eq!(interval.mean, 0.5);
        assert_relative_eq!(interval.margin, 0.20070508557018008);
    }
}
