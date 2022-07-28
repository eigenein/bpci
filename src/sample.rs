use std::fmt::Debug;

use num_traits::{Float, PrimInt, Unsigned};

use crate::{Error, Proportion, Result};

pub struct Sample<N, P> {
    pub(crate) size: N,
    proportion: Proportion<N, P>,
}

impl<N, P> Sample<N, P>
where
    N: Unsigned + Debug + PartialOrd,
{
    pub fn new(size: N, proportion: Proportion<N, P>) -> Result<Self> {
        if let Proportion::NSuccesses(n_successes) = &proportion {
            if n_successes > &size {
                return Err(Error::OutOfRange(format!(
                    "`{:?}` is out of range 0..{:?}",
                    n_successes, size
                )));
            }
        }
        Ok(Self { size, proportion })
    }
}

impl<N: PrimInt + Unsigned + Into<F>, F: Float> Sample<N, F> {
    pub fn p_hat(&self) -> F {
        match self.proportion {
            Proportion::PHat(p_hat) => p_hat,
            Proportion::NSuccesses(n_successes) => n_successes.into() / self.size.into(),
        }
    }
}
