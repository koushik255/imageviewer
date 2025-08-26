use ratatui::widgets::ListState;
use ratatui_image::thread::{ResizeRequest, ThreadProtocol};
use rayon::spawn;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::mpsc;

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
    pub image: ThreadProtocol,
    pub picker: Picker,
    pub photo_count: usize,
    pub current_photo: usize,
    pub filename: PathBuf,
    pub list_of_here: Vec<PathBuf>,
    pub next_photo: usize,
    pub prev_photo: usize,
    pub worker_tx: mpsc::Sender<ResizeRequest>,
    pub main_rx: mpsc::Receiver<AppEvent>,
    pub selected: usize,
    pub list_state: ListState,
    //pub photo_to_su::how: ,
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

        let (worker_tx, worker_rx) = mpsc::channel::<ResizeRequest>();
        let (main_tx, main_rx) = mpsc::channel();

        let dyn_img = image::ImageReader::open("./assets/painaura.jpeg")
            .unwrap()
            .decode()
            .unwrap();

        //12 + enum = 14

        let main_tx_clone = main_tx.clone();

        std::thread::spawn(move || {
            while let Ok(request) = worker_rx.recv() {
                let main_tx_clone = main_tx_clone.clone();

                // putting the hard work into a rayon spawn
                spawn(move || {
                    let result = request.resize_encode();
                    main_tx_clone.send(AppEvent::ImageReady(result)).unwrap();
                });
            }
        });

        let initial_protocol = picker.new_resize_protocol(dyn_img);

        Self {
            running: true,
            counter: 0,
            events: EventHandler::new(),
            dir_list: Vec::new(),
            image: ThreadProtocol::new(worker_tx.clone(), Some(initial_protocol)),
            picker,
            photo_count: 0,
            current_photo: 0,
            filename: PathBuf::new(),
            list_of_here: Vec::new(),
            next_photo: 0,
            prev_photo: 0,
            worker_tx,
            main_rx,
            selected: 0,
            list_state: ListState::default(),
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.list_dir_into_text();
        while self.running {
            terminal.draw(|frame| {
                let area = frame.area();
                //let image_widget = StatefulImage::default();
                //frame.render_stateful_widget(image_widget, area, &mut self.image);
                frame.render_widget(&mut self, area);
            })?;

            // checking for results (non-blocking)
            // do it here instead of the match
            // 2
            if let Ok(AppEvent::ImageReady(Ok(response))) = self.main_rx.try_recv() {
                //<-- main thread receives resized image result
                let _ = self.image.update_resized_protocol(response);
                // ->> updates threaProtocol with new resized image
            }

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
                    AppEvent::UpImage => self.go_up_image().await,
                    AppEvent::DownImage => self.go_down_image().await,
                    AppEvent::ImageReady(_) => {}
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
        self.increment_counter();

        //entries.sort();
        Ok(images)
    }

    pub fn list_dir_into_text(&mut self) {
        let stuff = self.show_dir().unwrap();
        self.dir_list = stuff.clone();

        self.increment_counter();
        self.list_of_here = stuff.clone();
    }

    pub fn one_by_one(&mut self) -> std::path::PathBuf {
        self.photo_count = self.list_of_here.len();
        // i only want to call the 2 functions above me once,
        // for now just click 'l' first before doing other stuff

        let c_p = self.current_photo;

        self.list_of_here[c_p].clone()
    }

    pub async fn go_up_image(&mut self) {
        self.current_photo += 1;
        self.load_new_image_thread().await;
    }

    pub async fn go_down_image(&mut self) {
        self.current_photo -= 1;
        self.load_new_image_thread().await;
    }

    pub fn show_image_screen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    pub async fn load_new_image_thread(&mut self) {
        let img_path = self.one_by_one();
        self.filename = img_path.clone();

        let dyn_img = image::ImageReader::open(img_path)
            .unwrap()
            .decode()
            .unwrap();

        //2
        let protocol = self.picker.new_resize_protocol(dyn_img);
        self.image = ThreadProtocol::new(self.worker_tx.clone(), Some(protocol));
        // sends resize request to woker via worker_tx
    }

    // before adding more things i want to try and make it faster
}
