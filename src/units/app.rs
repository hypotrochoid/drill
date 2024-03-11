use {
    super::{side_menu::SideMenuBar, top_menu::MenuBar, View},
    anyhow::Result,
    clap::Parser,
    eframe::{App, Frame, Storage},
    egui::{mutex::RwLock, Context},
    std::{fs, sync::Arc},
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct SeekerArgs {
    /// Name of the person to greet
    #[arg(short, long)]
    directory: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub enum ViewMode {
    Flashcard,
    Dictionary,
}

pub enum UpdateEvent {
    // LibraryModels,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SeekerApp {
    #[serde(skip_serializing, skip_deserializing)]
    current_mode: RwLock<ViewMode>,
    #[serde(skip_serializing, skip_deserializing)]
    label: RwLock<String>,
    #[serde(skip_serializing, skip_deserializing)]
    menu_bar: RwLock<MenuBar>,
    #[serde(skip_serializing, skip_deserializing)]
    side_menu: RwLock<SideMenuBar>,
    #[serde(skip_serializing, skip_deserializing)]
    ctx: RwLock<Option<Context>>,
    #[serde(skip_serializing, skip_deserializing)]
    want_update_rx: flume::Receiver<UpdateEvent>,
    #[serde(skip_serializing, skip_deserializing)]
    want_update_tx: flume::Sender<UpdateEvent>,
}

impl Default for SeekerApp {
    fn default() -> Self {
        Self::new(SeekerArgs {
            directory: Some(".".to_string()),
        })
        .unwrap()
    }
}

impl SeekerApp {
    pub fn new(args: SeekerArgs) -> Result<Self> {
        let (tx, rx) = flume::unbounded();

        Ok(Self {
            // Example stuff:
            // runtime: new_runtime(default_threads).expect("failed to make runtime"),
            current_mode: RwLock::new(ViewMode::Flashcard),
            label: RwLock::new("Hello World!".to_owned()),
            menu_bar: RwLock::new(MenuBar::default()),
            side_menu: RwLock::new(SideMenuBar::default()),
            ctx: RwLock::new(None),
            want_update_rx: rx,
            want_update_tx: tx,
        })
    }

    fn save(&self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        while let Ok(msg) = self.want_update_rx.try_recv() {
            match msg {}
        }

        let mode = self.side_menu.read().selected_mode.clone();
        match mode {
            ViewMode::Dictionary => self.draw_flashcard_view(ctx, frame),
            ViewMode::Flashcard => self.draw_flashcard_view(ctx, frame),
        }
    }

    fn draw_flashcard_view(&self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.menu_bar.write().ui(ui);
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| self.side_menu.write().ui(ui));

        egui::CentralPanel::default().show(ctx, |ui| {
            // self.library.write().ui(ui);
        });
    }

    fn save_directory(&self, dir: String) -> Result<()> {
        todo!()
    }
}

#[derive(Clone)]
pub struct AppWrapper {
    pub app: Arc<SeekerApp>,
}

impl AppWrapper {
    #[cfg(target_arch = "wasm32")]
    pub fn run_webgui(&self) {
        use eframe::WebOptions;

        let web_options = eframe::WebOptions::default();
        let mut cpy = self.clone();
        let gui_future = async {
            eframe::start_web(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| {
                    cpy.app.run_webgui(cc);
                    Box::new(cpy)
                }),
            )
            .await;
        };

        self.app.load_sample_data();

        wasm_bindgen_futures::spawn_local(gui_future);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn run_native(&self) {
        let native_options = eframe::NativeOptions::default();

        let cpy = self.clone();

        eframe::run_native("gondolier", native_options, Box::new(|_cc| Box::new(cpy)));
    }
}

impl App for AppWrapper {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.app.update(ctx, frame);
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        self.app.save(storage)
    }
}

mod test {
    use super::*;

    #[test]
    fn run_ui() {
        let mut app = AppWrapper {
            app: Arc::new(SeekerApp::new().expect("failed to start app")),
        };

        app.run_native();
    }
}
