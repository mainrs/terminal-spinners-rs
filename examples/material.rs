use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, MATERIAL};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new().spinner(&MATERIAL).text(text).start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
