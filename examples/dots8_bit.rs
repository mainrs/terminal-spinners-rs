use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, DOTS8_BIT};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new().spinner(&DOTS8_BIT).text(text).start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
