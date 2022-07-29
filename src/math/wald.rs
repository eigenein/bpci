//! [Normal approximation interval or Wald interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Normal_approximation_interval_or_Wald_interval).

use num_traits::Float;

use crate::{CenteredInterval, Sample};

pub trait Wald<F, N> {
    /// [Normal approximation interval or Wald interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Normal_approximation_interval_or_Wald_interval).
    fn wald(&self, z_score: F) -> CenteredInterval<F>;
}

impl<S, F, N> Wald<F, N> for S
where
    S: Sample<N, F>,
    N: Into<F>,
    F: Float,
{
    fn wald(&self, z_score: F) -> CenteredInterval<F> {
        let p_hat = self.p_hat();
        CenteredInterval {
            mean: p_hat,
            margin: z_score * (p_hat * (F::one() - p_hat) / self.size().into()).sqrt(),
        }
    }
}
