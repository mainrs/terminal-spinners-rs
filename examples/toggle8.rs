use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, TOGGLE8};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new().spinner(&TOGGLE8).text(text).start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
