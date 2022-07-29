pub mod error;
pub mod interval;
pub mod result;
pub mod sample;
pub mod wilson_score;

pub use self::error::Error;
pub use self::interval::*;
pub use self::result::Result;
pub use self::sample::*;
pub use self::wilson_score::*;

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
