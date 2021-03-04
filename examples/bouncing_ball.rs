use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, BOUNCING_BALL};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new()
        .spinner(&BOUNCING_BALL)
        .text(text)
        .start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
