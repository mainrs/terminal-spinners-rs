use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, DOTS4};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new().spinner(&DOTS4).text(text).start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
