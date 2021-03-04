use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, BETA_WAVE};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new().spinner(&BETA_WAVE).text(text).start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
