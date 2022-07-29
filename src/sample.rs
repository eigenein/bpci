//! Sample trait and structures.

pub mod n_successes;
pub mod p_hat;

pub use self::n_successes::*;
pub use self::p_hat::*;

pub trait Sample<N, P> {
    fn size(&self) -> N;
    fn p_hat(&self) -> P;
}
