use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, GROW_VERTICAL};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new()
        .spinner(&GROW_VERTICAL)
        .text(text)
        .start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
