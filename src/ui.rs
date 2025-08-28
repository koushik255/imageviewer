use ratatui::prelude::StatefulWidget;
use ratatui::widgets::ListItem;
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, List, ListState, Paragraph, Widget},
};

use ratatui_image::StatefulImage;

use crate::app::App;

impl Widget for &mut App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("{self.dir_list}")
            .title_alignment(ratatui::layout::Alignment::Center)
            .border_type(BorderType::Rounded);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Min(5),
                Constraint::Min(10),
                Constraint::Length(7),
            ])
            .split(area);

        let items1: Vec<ListItem> = self
            .dir_list
            .iter()
            .map(|path| {
                let display = path
                    .file_name()
                    .and_then(|os_str| os_str.to_str())
                    .unwrap_or("InvalidUTD()");

                ListItem::new(display.to_string())
            })
            .collect();

        //let mut llstate = ListState::default();
        let list = List::new(items1)
            .block(Block::bordered().title("List"))
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        StatefulWidget::render(list, chunks[0], buf, &mut self.list_state);

        let text = format!(
            "This is a tui template.\n\
                Counter: {}
                Current image: {:?}",
            self.counter, self.filename,
        );
        let image_block = Block::bordered()
            .title("image")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        let inner = image_block.inner(chunks[1]);

        let directory_listing = format!("{:?}", self.dir_list);

        let paragraph = Paragraph::new(text)
            .block(block.clone())
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        let dir_listing_para = Paragraph::new(directory_listing)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        paragraph.render(chunks[2], buf);
        // dir_listing_para.render(chunks[2], buf);

        image_block.render(chunks[1], buf);

        StatefulImage::default().render(inner, buf, &mut self.image);
        //image_space.render(inner, buf, &mut self.image);
    }
}
