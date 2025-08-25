use ratatui_image::StatefulImage;
use ratatui_image::protocol::StatefulProtocol;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};
use ratatui_image::picker::Picker;

use std::collections::HashSet;

/// Application.
//#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Counter.
    pub counter: u8,
    /// Event handler.
    pub events: EventHandler,
    pub dir_list: Vec<PathBuf>,
    pub image: StatefulProtocol,
    pub picker: Picker,
    pub photo_count: usize,
    pub current_photo: usize,
    pub filename: PathBuf,
    //pub photo_to_show: ,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let mut picker = Picker::from_query_stdio().unwrap();
        picker.set_background_color([0, 0, 0, 255]);

        let dyn_img = image::ImageReader::open("./assets/painaura.jpeg")
            .unwrap()
            .decode()
            .unwrap();

        let protocol = picker.new_resize_protocol(dyn_img);

        Self {
            running: true,
            counter: 0,
            events: EventHandler::new(),
            dir_list: Vec::new(),
            image: protocol,
            picker,
            photo_count: 0,
            current_photo: 0,
            filename: PathBuf::new(),
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| {
                let area = frame.area();
                //let image_widget = StatefulImage::default();
                //frame.render_stateful_widget(image_widget, area, &mut self.image);
                frame.render_widget(&mut self, area);
            })?;

            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Increment => self.increment_counter(),
                    AppEvent::Decrement => self.decrement_counter(),
                    AppEvent::Double => self.double_counter(),
                    AppEvent::Quit => self.quit(),
                    AppEvent::DirList => self.list_dir_into_text(),
                    AppEvent::UpImage => self.go_up_image(),
                    AppEvent::DownImage => self.go_down_image(),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Right => self.events.send(AppEvent::Increment),
            KeyCode::Left => self.events.send(AppEvent::Decrement),
            KeyCode::Up => self.events.send(AppEvent::Double),
            KeyCode::Char('l') => self.events.send(AppEvent::DirList),
            KeyCode::Char('u') => self.events.send(AppEvent::UpImage),
            KeyCode::Char('d') => self.events.send(AppEvent::DownImage),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }
    pub fn double_counter(&mut self) {
        self.counter = self.counter.saturating_add(2);
    }

    pub fn show_dir(&mut self) -> Result<Vec<PathBuf>, io::Error> {
        let entries = fs::read_dir("/home/koushikk/Downloads/")?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        //let mut images = Vec::new();

        let allowed: HashSet<&str> = ["jpg", "jpeg", "png"].into_iter().collect();

        let mut images: Vec<PathBuf> = entries
            .into_iter()
            //.filter(|file| file.extension().map_or(false, |file| file == "png"))
            .filter(|file| {
                file.extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| allowed.contains(&ext))
                    .unwrap_or(false)
            })
            .collect();
        // file.extension().map_or(false, |file| file == "jpg"))
        //.collect();

        images.sort();

        //entries.sort();
        Ok(images)
    }

    pub fn list_dir_into_text(&mut self) {
        let stuff = self.show_dir().unwrap();
        self.dir_list = stuff;
    }

    pub fn one_by_one(&mut self) -> std::path::PathBuf {
        let list_of_photos = self.show_dir().unwrap();
        self.photo_count = list_of_photos.len();
        // i only want to call the 2 functions above me once,

        let c_p = self.current_photo;

        list_of_photos[c_p].clone()
    }

    pub fn go_up_image(&mut self) {
        self.current_photo += 1;
        self.load_new_image();
    }

    pub fn go_down_image(&mut self) {
        self.current_photo -= 1;
        self.load_new_image();
    }

    pub fn show_image_screen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    pub fn load_new_image(&mut self) {
        let img = self.one_by_one();
        let file = self.one_by_one();
        self.filename = file;

        let dyn_img = image::ImageReader::open(img).unwrap().decode().unwrap();

        self.image = self.picker.new_resize_protocol(dyn_img)
    }
}
