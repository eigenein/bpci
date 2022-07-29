//! [Agresti–Coull interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Agresti%E2%80%93Coull_interval).

use num_traits::Float;

use crate::{CenteredInterval, NSuccessesSample, ProportionSample, Sample, Wald, WilsonScore};

pub trait AgrestiCoull<N, F> {
    /// [Agresti–Coull interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Agresti%E2%80%93Coull_interval).
    fn agresti_coull(&self, z_score: F) -> CenteredInterval<F>;
}

impl<N, F> AgrestiCoull<N, F> for ProportionSample<N, F>
where
    N: Copy + Into<F>,
    F: Float,
{
    fn agresti_coull(&self, z_score: F) -> CenteredInterval<F> {
        let p_tilde = self.wilson_score(z_score).mean;
        let n_tilde = self.size().into() + z_score * z_score;
        let corrected_sample = ProportionSample {
            size: n_tilde,
            proportion: p_tilde,
        };
        corrected_sample.wald(z_score)
    }
}

impl<N, F> AgrestiCoull<N, F> for NSuccessesSample<N>
where
    N: Copy + Into<F>,
    F: Float,
{
    fn agresti_coull(&self, z_score: F) -> CenteredInterval<F> {
        let z_squared = z_score * z_score;
        let n_tilde = self.size().into() + z_squared;
        let two = F::one() + F::one();
        let p_tilde = F::one() / n_tilde * (self.n_successes.into() + z_squared / two);
        let corrected_sample = ProportionSample {
            size: n_tilde,
            proportion: p_tilde,
        };
        corrected_sample.wald(z_score)
    }
}
