#[cfg(feature = "print")]
mod print;
#[cfg(feature = "print")]
pub use print::{SpinnerBuilder, SpinnerHandle};

mod spinners;
pub use spinners::*;

/// Data related to a spinner.
///
/// Each spinner consists of a number of frames and an interval. The interval is
/// used for animation and should be the amount of milliseconds between each
/// frame.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpinnerData<'a> {
    pub frames: &'a [&'a str],
    pub interval: u64,
}
