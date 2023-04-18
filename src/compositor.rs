use std::io::Stdout;

use crate::stats::Stats;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Terminal,
};

struct State {
    // Cursor row
    row: u16,
    // Cursor column
    column: u16,
}

impl State {
    pub fn new() -> Self {
        Self { row: 1, column: 1 }
    }
}

#[derive(Debug)]
struct Line {
    _line_number: usize,
    text: String,
    current_index: Option<usize>,
}

pub struct Compositor {
    lines: Vec<Line>,
    state: State,
    stats: Stats,
}

impl Compositor {
    pub fn new(text: &str) -> Self {
        let mut lines = Vec::new();

        for (index, line) in text.lines().enumerate() {
            let mut line = Line {
                _line_number: index,
                text: String::from(line),
                current_index: None,
            };

            if index == 0 {
                line.current_index = Some(0);
            }

            lines.push(line);
        }

        Self {
            lines,
            state: State::new(),
            stats: Stats::new(),
        }
    }

    fn lines_as_spans(&self) -> Vec<Spans<'_>> {
        self.lines
            .iter()
            .map(|l| {
                let v = vec![
                    // NOTE: should we add line numbers?
                    Span::styled(&l.text, Style::default().fg(Color::White).bg(Color::Reset)),
                ];

                Spans::from(v)
            })
            .collect::<Vec<_>>()
    }

    pub fn render(&self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        terminal
            .draw(|frame| {
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
                    .split(frame.size());

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
                            .border_type(BorderType::Rounded)
                            .borders(Borders::ALL),
                    )
                    .style(Style::default().fg(Color::Reset).bg(Color::Reset))
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true });

                let mid = Paragraph::new(self.lines_as_spans())
                    .block(
                        Block::default()
                            // .style(Style::default().bg(Color::Reset).fg(Color::Reset))
                            .border_type(BorderType::Rounded)
                            .borders(Borders::ALL),
                    )
                    .style(Style::default().fg(Color::Reset).bg(Color::Reset))
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: false });

                let text = vec![
                    Spans::from(vec![Span::styled(
                        "HELP",
                        Style::default().bg(Color::Reset).fg(Color::White),
                    )]),
                    Spans::from(vec![
                        Span::styled("esc : ", Style::default().bg(Color::Reset).fg(Color::Blue)),
                        Span::styled("quit\n", Style::default().bg(Color::Reset).fg(Color::White)),
                    ]),
                    Spans::from(vec![
                        Span::styled(
                            "ctrl-c : ",
                            Style::default().bg(Color::Reset).fg(Color::Blue),
                        ),
                        Span::styled("quit\n", Style::default().bg(Color::Reset).fg(Color::White)),
                    ]),
                ];

                let bot = Paragraph::new(text)
                    .block(
                        Block::default()
                            .style(Style::default().bg(Color::Reset).fg(Color::Reset))
                            .border_type(BorderType::Rounded)
                            .borders(Borders::ALL),
                    )
                    .style(Style::default().fg(Color::Reset).bg(Color::Reset))
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true });

                frame.render_widget(top, chunks[0]);
                frame.render_widget(mid, chunks[1]);
                let (x, y) = self.get_cursor(chunks[1]);
                frame.set_cursor(x, y);
                frame.render_widget(bot, chunks[2]);
            })
            .unwrap();
    }

    pub fn get_cursor(&self, area: Rect) -> (u16, u16) {
        (area.x + self.state.row, area.y + self.state.column)
    }

    pub fn type_char(&mut self, _ch: char) {
        self.stats.words += 1;
    }
}
