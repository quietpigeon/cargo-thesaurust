use crate::models::app::App;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

pub(crate) fn new(app: &mut App) -> List {
    let cloned_list = app.synonym_list.clone();
    let synonyms: Vec<ListItem> = cloned_list
        .items
        .iter()
        .map(|i| ListItem::new(i.clone()))
        .collect();

    List::new(synonyms)
        .block(Block::default().borders(Borders::ALL).title("Synonyms"))
        .style(Style::default().fg(Color::Green))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
}
