use crate::models::{app::App, input_mode::InputMode, thesaurus::Definition};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub(crate) fn new<'a>(
    app: &'a mut App,
    definitions: &[Definition],
    definition: &'a String,
) -> Paragraph<'a> {
    Paragraph::new(definition.to_string())
        .style(match app.input_mode {
            InputMode::SelectDefinition => Style::default().fg(Color::Yellow),
            _ => Style::default().fg(Color::Green),
        })
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title(format!(
            "Definition[{:}/{}]",
            app.definition_list.state.selected().unwrap() + 1,
            definitions.len()
        )))
}
