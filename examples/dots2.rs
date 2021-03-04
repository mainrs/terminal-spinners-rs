use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, DOTS2};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new().spinner(&DOTS2).text(text).start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
