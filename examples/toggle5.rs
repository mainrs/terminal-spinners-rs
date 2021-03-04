use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, TOGGLE5};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new().spinner(&TOGGLE5).text(text).start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
