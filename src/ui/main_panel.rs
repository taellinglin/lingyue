use eframe::{egui};
use egui::epaint::{self,Color32};
use egui::epaint::Hsva;
#[derive(Debug, Clone)]
enum TabName {
    Perform,
    Arrange,
    Compose,
    Instruments,
    Mixer,
    Settings,
}

struct Pane {
    nr: TabName,
}



impl Pane {
    pub fn new(nr: TabName) -> Self {
        Self { nr }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui_tiles::UiResponse {
        let hue = match &self.nr {
            TabName::Perform => 0.0,
            TabName::Arrange => 60.0,
            TabName::Compose => 120.0,
            TabName::Instruments => 180.0,
            TabName::Mixer => 240.0,
            TabName::Settings => 300.0,
        };

        let color = egui::epaint::Hsva::new(0.103 * self.nr.clone() as usize as f32, 0.5, 0.5, 1.0);
        ui.painter().rect_filled(ui.max_rect(), 0.0, color);
        

        

        let text = match &self.nr {
            TabName::Perform => "Perform",
            TabName::Arrange => "Arrange",
            TabName::Compose => "Compose",
            TabName::Instruments => "Instruments",
            TabName::Mixer => "Mixer",
            TabName::Settings => "Settings",
        };

        ui.label(format!("The contents of pane {}.", text));

        egui_tiles::UiResponse::None
    }
}


struct TreeBehavior {}

impl egui_tiles::Behavior<Pane> for TreeBehavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        match &pane.nr {
            TabName::Perform => egui::WidgetText::from("Perform"),
            TabName::Arrange => egui::WidgetText::from("Arrange"),
            TabName::Compose => egui::WidgetText::from("Compose"),
            TabName::Instruments => egui::WidgetText::from("Instruments"),
            TabName::Mixer => egui::WidgetText::from("Mixer"),
            TabName::Settings => egui::WidgetText::from("Settings"),
        }
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        pane.ui(ui)
    }
}

pub fn main_panel() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let mut tree = create_tree();

    eframe::run_simple_native("灵乐", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut behavior = TreeBehavior {};
            tree.ui(&mut behavior, ui);
        });
    })
}

fn create_tree() -> egui_tiles::Tree<Pane> {
    let mut tiles = egui_tiles::Tiles::default();

    let perform_pane = tiles.insert_pane(Pane::new(TabName::Perform));

    let arrange_timeline_pane = tiles.insert_pane(Pane::new(TabName::Arrange));
    let arrange_automation_pane = tiles.insert_pane(Pane::new(TabName::Arrange));

    let compose_piano_roll_pane = tiles.insert_pane(Pane::new(TabName::Compose));
    let compose_channel_editor_pane = tiles.insert_pane(Pane::new(TabName::Compose));

    let instruments_pane = tiles.insert_pane(Pane::new(TabName::Instruments));
    let mixer_pane = tiles.insert_pane(Pane::new(TabName::Mixer));

    let settings_pane = tiles.insert_pane(Pane::new(TabName::Settings));

    let perform_grid = {
        let mut children = Vec::new();
        for _ in 0..144 {
            children.push(tiles.insert_pane(Pane::new(TabName::Perform)));
        }
        tiles.insert_grid_tile(children)
    };

    let arrange_container = {
        let mut children = vec![arrange_timeline_pane, arrange_automation_pane];
        tiles.insert_vertical_tile(children)
    };

    let compose_container = {
        let mut children = vec![compose_piano_roll_pane, compose_channel_editor_pane];
        tiles.insert_vertical_tile(children)
    };

    let instruments_container = {
        let mut children = vec![instruments_pane];
        tiles.insert_vertical_tile(children)
    };

    let mixer_container = {
        let mut children = vec![mixer_pane];
        tiles.insert_horizontal_tile(children)
    };

    let settings_container = {
        let mut children = vec![settings_pane];
        tiles.insert_vertical_tile(children)
    };

    let mut tabs = Vec::new();
    tabs.push(perform_grid);
    tabs.push(arrange_container);
    tabs.push(compose_container);
    tabs.push(instruments_container);
    tabs.push(mixer_container);
    tabs.push(settings_container);

    let root = tiles.insert_tab_tile(tabs);

    egui_tiles::Tree::new(root, tiles)
}
