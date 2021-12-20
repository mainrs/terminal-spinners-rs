use crossterm::{cursor, queue, terminal};
use std::borrow::Cow;
use std::fmt::Display;
use std::{
    io::{stdout, Write},
    sync::mpsc::{channel, Receiver, Sender, TryRecvError},
    thread::{self, JoinHandle},
    time::Duration,
};
use terminal_log_symbols::colored::{
    ERROR_SYMBOL, INFO_SYMBOL, SUCCESS_SYMBOL, UNKNOWN_SYMBOL, WARNING_SYMBOL,
};
pub use terminal_spinner_data::*;

type Str = Cow<'static, str>;

#[derive(Copy, Clone)]
enum StopType {
    Done,
    Error,
    Info,
    Warn,
    Unknown,
}

impl Display for StopType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Done => write!(f, "{}", SUCCESS_SYMBOL),
            Self::Error => write!(f, "{}", ERROR_SYMBOL),
            Self::Info => write!(f, "{}", INFO_SYMBOL),
            Self::Warn => write!(f, "{}", WARNING_SYMBOL),
            Self::Unknown => write!(f, "{}", UNKNOWN_SYMBOL),
        }
    }
}

// Commands send through the mpsc channels to notify the render thread of certain events.
enum SpinnerCommand {
    /// Changes the text of the spinner. The change is visible once the spinner gets redrawn.
    ChangeText(Cow<'static, str>),

    // Commands that stop the spinner.
    Stop(Option<StopType>),
    StopAndClear,
}

// The internal representation of a spinner.
//
// Holds all the data needed to actually render the spinner on a render thread.
struct Spinner {
    data: &'static SpinnerData<'static>,
    text: Str,
    prefix: Str,
    rx: Receiver<SpinnerCommand>,
}

/// A builder for creating a terminal spinner.
#[derive(Clone, Default)]
pub struct SpinnerBuilder {
    spinner_data: Option<&'static SpinnerData<'static>>,
    text: Option<Str>,
    prefix: Option<Str>,
}

impl<'a> SpinnerBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The spinner animation to use.
    pub fn spinner(mut self, spinner: &'static SpinnerData<'static>) -> Self {
        self.spinner_data = Some(spinner);
        self
    }

    /// The text to show after the spinner animation.
    pub fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// The prefix to print before the actual spinning animation.
    ///
    /// # Note
    ///
    /// The prefix must not include newlines, as the library deletion does not account for those.
    pub fn prefix(mut self, prefix: impl Into<Cow<'static, str>>) -> Self {
        self.prefix = Some(prefix.into());
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
    pub fn start(self) -> SpinnerHandle {
        assert!(self.spinner_data.is_some());
        assert!(self.text.is_some());

        let (tx, rx) = channel();
        let spinner = Spinner {
            data: self.spinner_data.unwrap(),
            text: self.text.unwrap(),
            prefix: self.prefix.unwrap_or(Cow::Borrowed("")),
            rx,
        };
        spinner.start(tx)
    }
}

impl Spinner {
    fn start(mut self, tx: Sender<SpinnerCommand>) -> SpinnerHandle {
        let handle = thread::spawn(move || {
            let mut stdout = stdout();
            let mut symbol: Option<StopType> = None;

            // Cycle through the frames
            for &frame in self.data.frames.iter().cycle() {
                let mut should_clear_line = false;
                let mut should_stop_cycle_loop = false;

                match self.rx.try_recv() {
                    Ok(cmd) => match cmd {
                        SpinnerCommand::ChangeText(text) => self.text = text,
                        SpinnerCommand::Stop(s) => {
                            should_stop_cycle_loop = true;
                            symbol = s;
                        }
                        SpinnerCommand::StopAndClear => {
                            should_clear_line = true;
                            should_stop_cycle_loop = true;
                        }
                    },
                    Err(TryRecvError::Disconnected) => should_stop_cycle_loop = true,
                    _ => {} // We do not care about other types of errors.
                }

                // Continue with the animation.
                // 1. Delete current line.
                queue!(stdout, terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
                queue!(stdout, cursor::MoveToColumn(0)).unwrap();

                // 2. Check if we can early-stop.
                if should_stop_cycle_loop {
                    if !should_clear_line {
                        if let Some(symbol) = symbol {
                            writeln!(stdout, "{}{} {}", self.prefix, symbol, self.text).unwrap();
                        }
                    }

                    stdout.flush().unwrap();
                    break; // Breaks out of the animation loop
                }

                // 3. Print the new line.
                write!(stdout, "{}{}{}", self.prefix, frame, self.text).unwrap();
                stdout.flush().unwrap();

                // 4. Wait for the animation interval.
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
        self.tx
            .send(SpinnerCommand::Stop(Some(StopType::Done)))
            .unwrap();
        self.handle.join().unwrap();
    }

    /// Stops the spinner and renders an error symbol.
    pub fn error(self) {
        self.tx
            .send(SpinnerCommand::Stop(Some(StopType::Error)))
            .unwrap();
        self.handle.join().unwrap();
    }

    /// Stops the spinner and renders an information symbol.
    pub fn info(self) {
        self.tx
            .send(SpinnerCommand::Stop(Some(StopType::Info)))
            .unwrap();
        self.handle.join().unwrap();
    }

    /// Stops the spinner.
    pub fn stop(self) {
        self.tx.send(SpinnerCommand::Stop(None)).unwrap();
        self.handle.join().unwrap();
    }

    /// Stops the spinner and clears the line it was printed on.
    pub fn stop_and_clear(self) {
        self.tx.send(SpinnerCommand::StopAndClear).unwrap();
        self.handle.join().unwrap();
    }

    /// Changes the text of the spinner.
    pub fn text(&self, text: impl Into<Cow<'static, str>>) {
        self.tx
            .send(SpinnerCommand::ChangeText(text.into()))
            .unwrap();
    }

    /// Stops the spinner and renders a warning symbol.
    pub fn warn(self) {
        self.tx
            .send(SpinnerCommand::Stop(Some(StopType::Warn)))
            .unwrap();
        self.handle.join().unwrap();
    }

    /// Stops the spinner and renders an unknown symbol.
    pub fn unknown(self) {
        self.tx
            .send(SpinnerCommand::Stop(Some(StopType::Unknown)))
            .unwrap();
        self.handle.join().unwrap();
    }
}
