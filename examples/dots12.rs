use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, DOTS12};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new().spinner(&DOTS12).text(text).start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
