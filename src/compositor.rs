use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

pub struct Compositor {
    text: &'static str,
    lines: Vec<Spans<'static>>,
    area: Rect,
}

impl Compositor {
    pub fn new(text: &'static str, area: Rect) -> Self {
        let mut lines = Vec::new();

        for (index, line) in text.lines().enumerate() {
            let mut v = Vec::new();

            if index == 0 {
                let (first, remaining) = line.split_at(1);
                dbg!(first);
                v.push(Span::styled(
                    first,
                    Style::default()
                        .bg(Color::Rgb(0, 255, 0))
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ));
                v.push(Span::styled(
                    remaining,
                    Style::default().bg(Color::Reset).fg(Color::White),
                ));
            } else {
                v.push(Span::styled(
                    line,
                    Style::default().bg(Color::Reset).fg(Color::White),
                ));
            }

            lines.push(Spans::from(v));
        }

        Self { text, lines, area }
    }

    pub fn lines(&self) -> &[Spans<'static>] {
        &self.lines
    }
}
