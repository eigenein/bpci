use std::fmt::Debug;

use num_traits::{Float, Zero};

use crate::{Error, Result, Sample};

/// Represents a sample as size and number of successes.
pub struct NSuccessesSample<N> {
    size: N,
    n_successes: N,
}

impl<N: Zero + Debug + PartialOrd> NSuccessesSample<N> {
    pub fn new(size: N, n_successes: N) -> Result<Self> {
        if size < N::zero() {
            return Err(Error::OutOfRange(format!("size `{:?}` must be non-negative", size)));
        }
        if n_successes < N::zero() || n_successes > size {
            return Err(Error::OutOfRange(format!(
                "number of successes `{:?}` is out of range 0..=`{:?}`",
                n_successes, size
            )));
        }
        Ok(Self { size, n_successes })
    }
}

impl<N: Copy + Into<P>, P: Float> Sample<N, P> for NSuccessesSample<N> {
    fn size(&self) -> N {
        self.size
    }

    fn p_hat(&self) -> P {
        self.n_successes.into() / self.size.into()
    }
}
