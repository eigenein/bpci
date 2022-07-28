use std::fmt::Debug;

use num_traits::{One, Unsigned, Zero};

use crate::{Error, Result};

/// Specifies an input proportion,
pub enum Proportion<N, P> {
    /// As number of successes.
    NSuccesses(N),

    /// As sample proportion.
    #[non_exhaustive]
    PHat(P),
}

impl<N, P> Proportion<N, P>
where
    P: Zero + One + Debug + PartialOrd,
    N: Unsigned,
{
    pub fn new_with_p_hat(p_hat: P) -> Result<Self> {
        if p_hat >= P::zero() && p_hat <= P::one() {
            Ok(Self::PHat(p_hat))
        } else {
            Err(Error::OutOfRange(format!("`{:?}` is out of range `0..=1`", p_hat)))
        }
    }
}
