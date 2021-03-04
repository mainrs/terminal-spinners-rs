use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, CIRCLE_HALVES};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new()
        .spinner(&CIRCLE_HALVES)
        .text(text)
        .start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
