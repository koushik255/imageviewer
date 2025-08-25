use ratatui::prelude::StatefulWidget;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
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
            .title("imageviewer")
            .title_alignment(ratatui::layout::Alignment::Center)
            .border_type(BorderType::Rounded);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(5),
                Constraint::Min(10),
                Constraint::Length(7),
            ])
            .split(area);

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

        let image_space = StatefulImage::default();

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

        paragraph.render(chunks[0], buf);
        dir_listing_para.render(chunks[2], buf);
        image_block.render(chunks[1], buf);
        image_space.render(inner, buf, &mut self.image);
    }
}
