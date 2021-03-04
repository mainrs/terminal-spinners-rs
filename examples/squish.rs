use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, SQUISH};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new().spinner(&SQUISH).text(text).start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
