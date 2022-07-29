use std::fmt::Debug;

use num_traits::{Float, One, Unsigned, Zero};

use crate::{Error, Result};

pub trait Sample<N, P> {
    fn size(&self) -> N;
    fn p_hat(&self) -> P;
}

/// Represents a sample as size and number of successes.
pub struct NSuccessesSample<N> {
    size: N,
    n_successes: N,
}

impl<N: Unsigned + Debug + PartialOrd> NSuccessesSample<N> {
    pub fn new(size: N, n_successes: N) -> Result<Self> {
        if n_successes <= size {
            Ok(Self { size, n_successes })
        } else {
            Err(Error::OutOfRange(format!(
                "`{:?}` is out of range 0..=`{:?}`",
                n_successes, size
            )))
        }
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

/// Represents a sample with its size and proportion.
pub struct PHatSample<N, P> {
    size: N,
    proportion: P,
}

impl<N, P: Zero + One + Debug + PartialOrd> PHatSample<N, P> {
    pub fn new(size: N, proportion: P) -> Result<Self> {
        if proportion >= P::zero() && proportion <= P::one() {
            Ok(Self { size, proportion })
        } else {
            Err(Error::OutOfRange(format!("`{:?}` is out of range 0..=1", proportion)))
        }
    }
}

impl<N: Copy, P: Copy> Sample<N, P> for PHatSample<N, P> {
    fn size(&self) -> N {
        self.size
    }

    fn p_hat(&self) -> P {
        self.proportion
    }
}
