//! Binomial proportion confidence intervals.

pub mod error;
pub mod interval;
pub mod math;
pub mod result;
pub mod sample;

pub use self::error::Error;
pub use self::interval::*;
pub use self::math::*;
pub use self::result::Result;
pub use self::sample::*;

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}
