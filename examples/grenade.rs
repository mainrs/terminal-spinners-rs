use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, GRENADE};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new().spinner(&GRENADE).text(text).start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
