use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, BALLOON2};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new().spinner(&BALLOON2).text(text).start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
