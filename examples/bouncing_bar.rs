use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, BOUNCING_BAR};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new()
        .spinner(&BOUNCING_BAR)
        .text(text)
        .start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
