use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, DQPB};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new().spinner(&DQPB).text(text).start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
