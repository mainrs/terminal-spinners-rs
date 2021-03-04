use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, TOGGLE7};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new().spinner(&TOGGLE7).text(text).start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
