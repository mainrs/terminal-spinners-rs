use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, SIMPLE_DOTS};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new()
        .spinner(&SIMPLE_DOTS)
        .text(text)
        .start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
