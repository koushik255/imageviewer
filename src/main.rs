use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, enable_raw_mode},
};

use ratatui_image::picker::Picker;

use crate::app::App;
use std::io;

pub mod app;
pub mod event;
pub mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;
    let mut picker = Picker::from_query_stdio().unwrap();
    picker.set_background_color([0, 0, 0, 255]);

    //let backend = CrosstermBackend::new(stdout);

    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();

    result
}
