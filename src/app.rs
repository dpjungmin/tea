use std::{
    io::{self, Stdout},
    process,
};

use crate::compositor::Compositor;

use anyhow::Result;

#[cfg(not(windows))]
use {signal_hook::consts::signal, signal_hook_tokio::Signals};
#[cfg(windows)]
type Signals = futures_util::stream::Empty<()>;

pub struct App {
    compositor: Compositor,
    terminal: tui::Terminal<tui::backend::CrosstermBackend<Stdout>>,
    signals: Signals,
    exit_code: i32,
}

impl App {
    pub fn new(text: &'static str) -> Result<Self> {
        let compositor = Compositor::new(text);
        let backend = tui::backend::CrosstermBackend::new(io::stdout());
        let terminal = tui::Terminal::new(backend)?;

        #[cfg(windows)]
        let signals = futures_util::stream::empty();
        #[cfg(not(windows))]
        let signals = Signals::new([signal::SIGTSTP, signal::SIGCONT, signal::SIGUSR1])?;

        let app = Self {
            compositor,
            terminal,
            signals,
            exit_code: 0,
        };

        Ok(app)
    }

    fn claim_term(&mut self) -> io::Result<()> {
        use crossterm::{
            event::{
                EnableFocusChange, EnableMouseCapture, KeyboardEnhancementFlags,
                PushKeyboardEnhancementFlags,
            },
            execute,
            terminal::{self, Clear, ClearType, EnterAlternateScreen},
        };

        terminal::enable_raw_mode()?;

        let buf = self.terminal.backend_mut();

        #[rustfmt::skip]
        execute!(buf, EnterAlternateScreen, EnableMouseCapture, EnableFocusChange)?;
        execute!(buf, Clear(ClearType::All))?;

        if terminal::supports_keyboard_enhancement().is_ok() {
            execute!(
                buf,
                PushKeyboardEnhancementFlags(
                    KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                        | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                )
            )?;
        }

        Ok(())
    }

    fn restore_term(&mut self) -> io::Result<()> {
        use crossterm::{
            cursor,
            event::{DisableFocusChange, DisableMouseCapture, PopKeyboardEnhancementFlags},
            execute,
            terminal::{self, LeaveAlternateScreen},
        };

        terminal::disable_raw_mode()?;

        let buf = self.terminal.backend_mut();

        #[rustfmt::skip]
        execute!(buf, LeaveAlternateScreen, DisableMouseCapture, DisableFocusChange, cursor::Show)?;

        if terminal::supports_keyboard_enhancement().is_ok() {
            execute!(buf, PopKeyboardEnhancementFlags)?;
        }

        Ok(())
    }

    async fn render(&mut self) {
        self.compositor.render(&mut self.terminal);
    }

    #[cfg(windows)]
    // no signal handling available on windows
    pub async fn handle_signal(&mut self, _signal: ()) {}

    #[cfg(not(windows))]
    pub async fn handle_signal(&mut self, signal: i32) {
        match signal {
            signal::SIGTSTP => {
                self.restore_term().unwrap();

                // SAFETY:
                //
                // - helix must have permissions to send signals to all processes in its signal
                //   group, either by already having the requisite permission, or by having the
                //   user's UID / EUID / SUID match that of the receiving process(es).
                let res = unsafe {
                    // A pid of 0 sends the signal to the entire process group, allowing the user to
                    // regain control of their terminal if the editor was spawned under another process
                    // (e.g. when running `git commit`).
                    //
                    // We have to send SIGSTOP (not SIGTSTP) to the entire process group, because,
                    // as mentioned above, the terminal will get stuck if `helix` was spawned from
                    // an external process and that process waits for `helix` to complete. This may
                    // be an issue with signal-hook-tokio, but the author of signal-hook believes it
                    // could be a tokio issue instead:
                    // https://github.com/vorner/signal-hook/issues/132
                    libc::kill(0, signal::SIGSTOP)
                };

                if res != 0 {
                    let err = io::Error::last_os_error();
                    eprintln!("{}", err);
                    let raw_os_error = err.raw_os_error().unwrap_or(1);
                    process::exit(raw_os_error);
                }
            }
            signal::SIGCONT => {
                self.claim_term().unwrap();
                self.terminal.clear().expect("couldn't clear terminal");
                self.render().await;
            }
            signal::SIGUSR1 => {
                self.render().await;
            }
            _ => unreachable!(),
        }
    }

    async fn handle_event(&mut self, event: Result<crossterm::event::Event, crossterm::ErrorKind>) {
        use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

        let should_redraw = match event.unwrap() {
            Event::Resize(_columns, _rows) => true,
            Event::Key(KeyEvent {
                kind: KeyEventKind::Release,
                ..
            }) => false,
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => {
                self.restore_term().unwrap();
                process::exit(0);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                self.restore_term().unwrap();
                process::exit(0);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                ..
            }) => {
                self.compositor.type_char(ch);
                true
            }
            _event => {
                // TODO: add event handler to compositor
                false
            }
        };

        if should_redraw {
            self.render().await;
        }
    }

    async fn event_loop_until_idle<S>(&mut self, input_stream: &mut S) -> bool
    where
        S: futures_util::Stream<Item = crossterm::Result<crossterm::event::Event>> + Unpin,
    {
        use futures_util::StreamExt;

        loop {
            tokio::select! {
                biased;

                Some(signal) = self.signals.next() => {
                    self.handle_signal(signal).await;
                }

                Some(event) = input_stream.next() => {
                    self.handle_event(event).await;
                }
            }
        }
    }

    async fn event_loop<S>(&mut self, input_stream: &mut S)
    where
        S: futures_util::Stream<Item = crossterm::Result<crossterm::event::Event>> + Unpin,
    {
        self.render().await;

        loop {
            if !self.event_loop_until_idle(input_stream).await {
                break;
            }
        }
    }

    pub async fn run<S>(&mut self, input_stream: &mut S) -> Result<i32>
    where
        S: futures_util::Stream<Item = crossterm::Result<crossterm::event::Event>> + Unpin,
    {
        self.claim_term()?;

        // TODO: set panic hook that exits the screen and disables raw mode

        self.event_loop(input_stream).await;

        self.restore_term()?;

        Ok(self.exit_code)
    }
}
