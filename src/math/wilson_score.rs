//! Wilson score intervals.

use std::ops::{Add, Div};

use num_traits::{Float, One};

use crate::interval::{BoundedInterval, CenteredInterval};
use crate::Sample;

pub trait WilsonScore<F, N> {
    /// [Wilson score interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Wilson_score_interval).
    fn wilson_score(&self, z_level: F) -> CenteredInterval<F>;

    /// [Wilson score interval with continuity correction](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Wilson_score_interval_with_continuity_correction)
    /// as per [Wallis (2021)](https://www.routledge.com/Statistics-in-Corpus-Linguistics-Research-A-New-Approach/Wallis/p/book/9781138589384).
    fn wilson_score_with_cc(&self, z_level: F) -> BoundedInterval<F>;
}

impl<S, F, N> WilsonScore<F, N> for S
where
    S: Sample<N, F>,
    F: Float,
    N: Into<F>,
{
    fn wilson_score(&self, z_level: F) -> CenteredInterval<F> {
        let (one, two, four) = one_two_four();

        let sample_size = self.size().into();
        let p_hat = self.p_hat();
        let (a, b) = a_b(sample_size, z_level);

        CenteredInterval {
            mean: b * (p_hat + a / two),
            margin: z_level
                * b
                * (p_hat * (one - p_hat) / sample_size + a / sample_size / four).sqrt(),
        }
    }

    fn wilson_score_with_cc(&self, z_level: F) -> BoundedInterval<F> {
        let (one, two, four) = one_two_four();

        let sample_size = self.size().into();
        let (lower_p_hat, upper_p_hat) = {
            let p_hat = self.p_hat();
            let p_hat_correction = F::one() / (sample_size + sample_size);
            ((p_hat - p_hat_correction).max(F::zero()), (p_hat + p_hat_correction).min(one))
        };
        let (a, b) = a_b(sample_size, z_level);
        let half_a = a / two;
        let quarter_sized_a = a / sample_size / four;

        BoundedInterval {
            lower: b * (lower_p_hat + half_a)
                - z_level
                    * b
                    * (lower_p_hat * (one - lower_p_hat) / sample_size + quarter_sized_a).sqrt(),
            upper: b * (upper_p_hat + half_a)
                + z_level
                    * b
                    * (upper_p_hat * (one - upper_p_hat) / sample_size + quarter_sized_a).sqrt(),
        }
    }
}

#[inline]
fn one_two_four<F: Copy + One + Add<Output = F>>() -> (F, F, F) {
    let one = F::one();
    let two = F::one() + F::one();
    let four = two + two;
    (one, two, four)
}

#[inline]
fn a_b<F>(sample_size: F, z_level: F) -> (F, F)
where
    F: One + Copy + Div<Output = F> + Add<Output = F>,
{
    let one = F::one();
    let a = z_level * z_level / sample_size;
    let b = one / (one + a);
    (a, b)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::*;

    #[test]
    /// Verified with Wolfram Alpha.
    fn wilson_score_ok() {
        let sample = NSuccessesSample::new(20, 8).unwrap();
        let interval = sample.wilson_score(1.960);
        assert_relative_eq!(interval.lower(), 0.2188039674141927);
        assert_relative_eq!(interval.upper(), 0.613422057684794);
    }

    #[test]
    /// Verified with Wolfram Alpha.
    fn wilson_score_with_cc_ok() {
        let sample = PHatSample::new(20, 0.4).unwrap();
        let interval = sample.wilson_score_with_cc(1.960);
        assert_relative_eq!(interval.lower, 0.19976843301470645);
        assert_relative_eq!(interval.upper, 0.6358867106798909);
    }
}
