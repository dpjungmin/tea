use std::{collections::VecDeque, io, time::Duration};

use anyhow::Result;
use crossterm::{
    event::{
        self, poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, Paragraph, Wrap},
    Terminal,
};

pub struct App<'a> {
    text: &'a str,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            text: crate::EXAMPLE_TEXT,
        }
    }
}

impl<'a> App<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    pub fn run(self) -> Result<()> {
        // Open app
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        'outer: loop {
            // Draw widgets
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(10),
                            Constraint::Percentage(80),
                            Constraint::Percentage(10),
                        ]
                        .as_ref(),
                    )
                    .horizontal_margin(2)
                    .vertical_margin(1)
                    .split(f.size());

                let text = vec![Spans::from(vec![
                    Span::styled(
                        "T",
                        Style::default()
                            .bg(Color::Reset)
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        "erminal-typing ",
                        Style::default().bg(Color::Reset).fg(Color::Yellow),
                    ),
                    Span::styled(
                        "E",
                        Style::default()
                            .bg(Color::Reset)
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        "xercise ",
                        Style::default().bg(Color::Reset).fg(Color::Yellow),
                    ),
                    Span::styled(
                        "A",
                        Style::default()
                            .bg(Color::Reset)
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        "pplication",
                        Style::default().bg(Color::Reset).fg(Color::Yellow),
                    ),
                ])];

                let top = Paragraph::new(text)
                    .block(
                        Block::default()
                            .style(Style::default().bg(Color::Reset).fg(Color::Reset))
                            .borders(Borders::NONE),
                    )
                    .style(Style::default().fg(Color::Reset).bg(Color::Reset))
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true });

                let mut text = self
                    .text
                    .lines()
                    .into_iter()
                    .map(|l| {
                        Spans::from(vec![Span::styled(
                            l,
                            Style::default().bg(Color::Reset).fg(Color::White),
                        )])
                    })
                    .collect::<VecDeque<_>>();

                text.push_front(Spans::from(vec![Span::styled(
                    " ",
                    Style::default()
                        .bg(Color::Green)
                        .fg(Color::White)
                        .add_modifier(Modifier::RAPID_BLINK),
                )]));

                text.push_back(Spans::from(vec![Span::styled(
                    " ",
                    Style::default()
                        .bg(Color::Red)
                        .fg(Color::White)
                        .add_modifier(Modifier::RAPID_BLINK),
                )]));

                let mid = Paragraph::new(Vec::from(text))
                    .block(
                        Block::default()
                            .title("example.rs")
                            .style(Style::default().bg(Color::Reset).fg(Color::Reset))
                            .border_type(BorderType::Rounded)
                            .borders(Borders::ALL),
                    )
                    .style(Style::default().fg(Color::Reset).bg(Color::Reset))
                    .alignment(Alignment::Left);

                let text = vec![
                    Spans::from(vec![Span::styled(
                        "HELP",
                        Style::default().bg(Color::Reset).fg(Color::White),
                    )]),
                    Spans::from(vec![
                        Span::styled("q : ", Style::default().bg(Color::Reset).fg(Color::Blue)),
                        Span::styled("quit\n", Style::default().bg(Color::Reset).fg(Color::White)),
                    ]),
                ];

                let bot = Paragraph::new(text)
                    .block(
                        Block::default()
                            .style(Style::default().bg(Color::Reset).fg(Color::Reset))
                            .borders(Borders::NONE),
                    )
                    .style(Style::default().fg(Color::Reset).bg(Color::Reset))
                    .alignment(Alignment::Right)
                    .wrap(Wrap { trim: true });

                f.render_widget(top, chunks[0]);
                f.render_widget(mid, chunks[1]);
                f.render_widget(bot, chunks[2]);
            })?;

            // `poll()` waits for an `Event` for a given time period
            if poll(Duration::from_millis(500))? {
                if let Event::Key(event) = read()? {
                    match event.code {
                        KeyCode::Char('q') => break 'outer,
                        _ => {}
                    }
                }
                // match read()? {
                //     Event::FocusGained => println!("FocusGained"),
                //     Event::FocusLost => println!("FocusLost"),
                //     Event::Key(event) => println!("{:?}", event),
                //     Event::Mouse(event) => println!("{:?}", event),
                //     #[cfg(feature = "bracketed-paste")]
                //     Event::Paste(data) => println!("Pasted {:?}", data),
                //     Event::Resize(width, height) => println!("New size {}x{}", width, height),
                //     _ => {}
                // }
            } else {
                // Timeout expired and no `Event` is available
            }
        }

        // Close app
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

        Ok(())
    }
}
