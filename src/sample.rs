//! Sample trait and structures.

pub mod n_successes;
pub mod proportion;

pub use self::n_successes::*;
pub use self::proportion::*;

pub trait Sample<N, P> {
    fn size(&self) -> N;
    fn p_hat(&self) -> P;
}
