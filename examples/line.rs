use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, LINE};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new().spinner(&LINE).text(text).start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
