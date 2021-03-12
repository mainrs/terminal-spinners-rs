use crate::SpinnerData;
use crossterm::{cursor, queue, terminal};
use std::borrow::Cow;
use std::{
    io::{stdout, Write},
    sync::mpsc::{channel, Receiver, Sender, TryRecvError},
    thread::{self, JoinHandle},
    time::Duration,
};
use terminal_log_symbols::colored::{ERROR_SYMBOL, INFO_SYMBOL, SUCCESS_SYMBOL, WARNING_SYMBOL};

// Commands send through the mpsc channels to notify the render thread of certain events.
enum SpinnerCommand {
    ChangeText(String),
    Done,
    Error,
    Info,
    Stop,
    Warn,
}

// The internal representation of a spinner.
//
// Holds all the data needed to actually render the spinner on a render thread.
struct Spinner {
    data: &'static SpinnerData<'static>,
    text: Cow<'static, str>,
    rx: Receiver<SpinnerCommand>,
}

/// A builder for creating a terminal spinner.
#[derive(Clone, Default)]
pub struct SpinnerBuilder {
    spinner_data: Option<&'static SpinnerData<'static>>,
    text: Option<&'static str>,
}

impl<'a> SpinnerBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The spinner animation to use.
    pub fn spinner(&'a mut self, spinner: &'static SpinnerData<'static>) -> &'a mut Self {
        self.spinner_data = Some(spinner);
        self
    }

    /// The text to show after the spinner animation.
    pub fn text(&'a mut self, text: &'static str) -> &'a mut Self {
        self.text = Some(text);
        self
    }

    /// Starts the spinner and renders it on a separate thread.
    ///
    /// # Returns
    ///
    /// A `SpinnerHandle`, allowing for further control of the spinner after it gets rendered.
    ///
    /// # Panics
    ///
    /// If no text and spinner have been set.
    pub fn start(&self) -> SpinnerHandle {
        assert!(self.spinner_data.is_some());
        assert!(self.text.is_some());

        let (tx, rx) = channel();
        let spinner = Spinner {
            data: self.spinner_data.unwrap(),
            text: self.text.unwrap().into(),
            rx,
        };
        spinner.start(tx)
    }
}

impl Spinner {
    fn start(mut self, tx: Sender<SpinnerCommand>) -> SpinnerHandle {
        let handle = thread::spawn(move || {
            let mut stdout = stdout();

            // Use a number and the lower four bits to see what command has been send. Makes the if statements easier.
            // From low to high: done, error, info, warning.
            let mut cmd_flags = 0u8;

            // Cycle through the frames
            for &frame in self.data.frames.iter().cycle() {
                let mut should_stop_cycle_loop = false;

                loop {
                    match self.rx.try_recv() {
                        Ok(cmd) => match cmd {
                            SpinnerCommand::ChangeText(text) => {
                                self.text = text.into();
                            }
                            SpinnerCommand::Done => {
                                cmd_flags |= 0b1;
                            }
                            SpinnerCommand::Error => {
                                cmd_flags |= 0b10;
                            }
                            SpinnerCommand::Info => {
                                cmd_flags |= 0b100;
                            }
                            SpinnerCommand::Warn => {
                                cmd_flags |= 0b1000;
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

                // Check if we need to print an emoji or the current frame.
                if cmd_flags != 0 {
                    let emoji_to_write = match cmd_flags {
                        0b0001 => SUCCESS_SYMBOL,
                        0b0010 => ERROR_SYMBOL,
                        0b0100 => INFO_SYMBOL,
                        0b1000 => WARNING_SYMBOL,
                        _ => unreachable!(),
                    };
                    write!(stdout, "{} {}", emoji_to_write, self.text).unwrap();
                    should_stop_cycle_loop = true;
                } else {
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

/// A handle to a running spinner.
///
/// Can be used to send commands to the render thread.
pub struct SpinnerHandle {
    handle: JoinHandle<()>,
    tx: Sender<SpinnerCommand>,
}

impl SpinnerHandle {
    /// Stops the spinner and renders a success symbol.
    pub fn done(self) {
        self.tx.send(SpinnerCommand::Done).unwrap();
        self.stop();
    }

    /// Stops the spinner and renders an error symbol.
    pub fn error(self) {
        self.tx.send(SpinnerCommand::Error).unwrap();
        self.stop();
    }

    /// Stops the spinner and renders an information symbol.
    pub fn info(self) {
        self.tx.send(SpinnerCommand::Info).unwrap();
        self.stop();
    }

    /// Stops the spinner.
    pub fn stop(self) {
        self.tx.send(SpinnerCommand::Stop).unwrap();
        self.handle.join().unwrap();
    }

    /// Changes the text of the spinner.
    pub fn text(&self, text: impl ToString) {
        self.tx
            .send(SpinnerCommand::ChangeText(text.to_string()))
            .unwrap();
    }

    /// Stops the spinner and renders a warning symbol.
    pub fn warn(self) {
        self.tx.send(SpinnerCommand::Warn).unwrap();
        self.stop();
    }
}
