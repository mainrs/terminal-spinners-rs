use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, SMILEY};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new().spinner(&SMILEY).text(text).start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
