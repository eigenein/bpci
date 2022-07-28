use num_traits::{Float, PrimInt, Unsigned};

use crate::Proportion;

pub struct Sample<N, P> {
    pub size: N,
    pub proportion: Proportion<N, P>,
}

impl<N: PrimInt + Unsigned + Into<F>, F: Float> Sample<N, F> {
    pub fn p_hat(&self) -> F {
        match self.proportion {
            Proportion::PHat(p_hat) => p_hat,
            Proportion::NSuccesses(n_successes) => n_successes.into() / self.size.into(),
        }
    }
}
