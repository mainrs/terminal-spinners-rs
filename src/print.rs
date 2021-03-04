use crate::SpinnerData;
use crossterm::{cursor, queue, style::Colorize, terminal};
use std::{
    io::{stdout, Write},
    sync::mpsc::{channel, Receiver, Sender, TryRecvError},
    thread::{self, JoinHandle},
    time::Duration,
};
use terminal_emoji::Emoji;

const ERROR_EMOJI: Emoji = Emoji::new("✖", "×");
const INFO_EMOJI: Emoji = Emoji::new("ℹ", "i");
const SUCCESS_EMOJI: Emoji = Emoji::new("✔", "√");
const WARNING_EMOJI: Emoji = Emoji::new("⚠", "‼");

enum SpinnerCommand {
    Done,
    Error,
    Info,
    Stop,
    Warn,
}

struct Spinner {
    data: &'static SpinnerData<'static>,
    text: &'static str,
    rx: Receiver<SpinnerCommand>,
}

#[derive(Clone, Default)]
pub struct SpinnerBuilder {
    spinner_data: Option<&'static SpinnerData<'static>>,
    text: Option<&'static str>,
}

impl<'a> SpinnerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spinner(&'a mut self, spinner: &'static SpinnerData<'static>) -> &'a mut Self {
        self.spinner_data = Some(spinner);
        self
    }

    pub fn text(&'a mut self, text: &'static str) -> &'a mut Self {
        self.text = Some(text);
        self
    }

    pub fn start(&self) -> SpinnerHandle {
        assert!(self.spinner_data.is_some());
        assert!(self.text.is_some());

        let (tx, rx) = channel();
        let spinner = Spinner {
            data: self.spinner_data.unwrap(),
            text: self.text.unwrap(),
            rx,
        };
        spinner.start(tx)
    }
}

impl Spinner {
    pub fn start(self, tx: Sender<SpinnerCommand>) -> SpinnerHandle {
        let handle = thread::spawn(move || {
            // Create the terminal instance used for rendering.
            let mut stdout = stdout();

            let mut is_done = false;
            let mut is_error = false;
            let mut is_info = false;
            let mut is_warn = false;

            // Cycle through the frames
            for &frame in self.data.frames.iter().cycle() {
                let mut should_stop_cycle_loop = false;

                loop {
                    match self.rx.try_recv() {
                        Ok(cmd) => match cmd {
                            SpinnerCommand::Done => {
                                is_done = true;
                            }
                            SpinnerCommand::Error => {
                                is_error = true;
                            }
                            SpinnerCommand::Info => {
                                is_info = true;
                            }
                            SpinnerCommand::Warn => {
                                is_warn = true;
                            }
                            SpinnerCommand::Stop => {
                                should_stop_cycle_loop = true;
                            }
                        },
                        Err(TryRecvError::Empty) => break,
                        Err(TryRecvError::Disconnected) => {
                            should_stop_cycle_loop = true;
                        }
                    }
                }

                // Delete old line.
                queue!(stdout, terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
                queue!(stdout, cursor::MoveToColumn(0)).unwrap();

                if is_warn {
                    write!(
                        stdout,
                        "{} {}",
                        WARNING_EMOJI.to_string().yellow(),
                        self.text
                    )
                    .unwrap();
                    should_stop_cycle_loop = true;
                } else if is_error {
                    write!(stdout, "{} {}", ERROR_EMOJI.to_string().red(), self.text).unwrap();
                    should_stop_cycle_loop = true;
                } else if is_done {
                    write!(
                        stdout,
                        "{} {}",
                        SUCCESS_EMOJI.to_string().green(),
                        self.text
                    )
                    .unwrap();
                    should_stop_cycle_loop = true;
                } else if is_info {
                    write!(stdout, "{} {}", INFO_EMOJI.to_string().blue(), self.text).unwrap();
                    should_stop_cycle_loop = true;
                } else {
                    // Write frame to output, followed by the message
                    write!(stdout, "{}{}", frame, self.text).unwrap();
                }

                // Flush output.
                stdout.flush().unwrap();

                if should_stop_cycle_loop {
                    break;
                }

                // Wait for the animation interval.
                std::thread::sleep(Duration::from_millis(self.data.interval));
            }
        });
        SpinnerHandle { handle, tx }
    }
}

pub struct SpinnerHandle {
    handle: JoinHandle<()>,
    tx: Sender<SpinnerCommand>,
}

impl SpinnerHandle {
    pub fn done(self) {
        self.tx.send(SpinnerCommand::Done).unwrap();
        self.stop();
    }

    pub fn error(self) {
        self.tx.send(SpinnerCommand::Error).unwrap();
        self.stop();
    }

    pub fn info(self) {
        self.tx.send(SpinnerCommand::Info).unwrap();
        self.stop();
    }

    pub fn stop(self) {
        self.tx.send(SpinnerCommand::Stop).unwrap();
        self.handle.join().unwrap();
    }

    pub fn warn(self) {
        self.tx.send(SpinnerCommand::Warn).unwrap();
        self.stop();
    }
}
