use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::{Rect};
use ratatui::style::{Color, Style, Modifier};
use ratatui::widgets::{Block, Borders, BorderType, Paragraph};
use ratatui::widgets::Widget;
use ratatui::text::{Span, Line};
use crate::slash_command::{built_in_slash_commands};
use super::{BottomPane, BottomPaneView};

pub(crate) struct HelpModalView {
    is_complete: bool,
}

impl HelpModalView {
    pub fn new() -> Self {
        Self { is_complete: false }
    }
}

impl<'a> BottomPaneView<'a> for HelpModalView {
    fn handle_key_event(&mut self, _pane: &mut BottomPane<'a>, key_event: KeyEvent) {
        use crossterm::event::{KeyCode};
        match key_event.code {
            KeyCode::Esc => self.is_complete = true,
            KeyCode::Char('q') | KeyCode::Char('Q') => self.is_complete = true,
            _ => {}
        }
    }

    fn is_complete(&self) -> bool {
        self.is_complete
    }

    fn calculate_required_height(&self, _area: &Rect) -> u16 {
        let n = built_in_slash_commands().len() as u16;
        n + 4 
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let style = Style::default().bg(Color::Blue).fg(Color::White);
        let bold = style.add_modifier(Modifier::BOLD);

        let mut lines = vec![
            Line::from(vec![Span::styled("Available commands", bold)]),
            Line::from(""),
            Line::from(vec![Span::styled("Slash‑commands", bold)]),
        ];

        // Dynamically add slash commands
        let mut commands: Vec<(String, String)> = built_in_slash_commands()
            .values()
            .map(|cmd| (format!("/{}", cmd.command()), cmd.description().to_string()))
            .collect();
        commands.sort_by(|a, b| a.0.cmp(&b.0));
        for (cmd, desc) in commands {
            lines.push(Line::from(format!("{cmd} – {desc}")));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled("Keyboard shortcuts", bold)]));
        lines.push(Line::from("Enter – send message"));
        lines.push(Line::from("Ctrl+J – insert newline"));
        lines.push(Line::from("Up/Down – scroll prompt history"));
        lines.push(Line::from("Esc(✕2) – interrupt current action"));
        lines.push(Line::from("Ctrl+C – quit Codex"));

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .title("Press esc or q to close")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(style),
            )
            .style(style);

        paragraph.render(area, buf);
    }
} 