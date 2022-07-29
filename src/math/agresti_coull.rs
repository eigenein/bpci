//! [Agresti–Coull interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Agresti%E2%80%93Coull_interval).

use num_traits::Float;

use crate::{CenteredInterval, PHatSample, Sample, Wald, WilsonScore};

pub trait AgrestiCoull<F, N> {
    /// [Agresti–Coull interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Agresti%E2%80%93Coull_interval).
    fn agresti_coull(&self, z_score: F) -> CenteredInterval<F>;
}

impl<S, F, N> AgrestiCoull<F, N> for S
where
    S: Sample<N, F>,
    N: Into<F>,
    F: Float,
{
    fn agresti_coull(&self, z_score: F) -> CenteredInterval<F> {
        let p_tilde = self.wilson_score(z_score).mean;
        let n_tilde = self.size().into() + z_score * z_score;
        let corrected_sample = PHatSample {
            size: n_tilde,
            proportion: p_tilde,
        };
        corrected_sample.wald(z_score)
    }
}
