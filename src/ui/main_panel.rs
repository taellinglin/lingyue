use egui::{containers::*, *};

pub struct MainPanel {
    active_tab: usize,
}

impl MainPanel {
    pub fn new() -> Self {
        MainPanel { active_tab: 0 }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let tabs = vec!["Perform", "Arrange", "Compose", "Instruments", "Mixer", "Settings"];

        // Create the tab bar
        let response = ui.horizontal(|ui| {
            for (i, tab) in tabs.iter().enumerate() {
                let is_active = i == self.active_tab;
                if ui.tab_button(tab).selected(is_active).clicked() {
                    self.active_tab = i;
                }
            }
        });

        // Create the tab content
        match self.active_tab {
            0 => self.perform_tab(ui),
            1 => self.arrange_tab(ui),
            2 => self.compose_tab(ui),
            3 => self.instruments_tab(ui),
            4 => self.mixer_tab(ui),
            5 => self.settings_tab(ui),
            _ => unreachable!(),
        }
    }

    fn perform_tab(&mut self, ui: &mut Ui) {
        // TODO: Add content for the Perform tab
        ui.label("Perform Tab");
    }

    fn arrange_tab(&mut self, ui: &mut Ui) {
        // TODO: Add content for the Arrange tab
        ui.label("Arrange Tab");
    }

    fn compose_tab(&mut self, ui: &mut Ui) {
        // TODO: Add content for the Compose tab
        ui.label("Compose Tab");
    }

    fn instruments_tab(&mut self, ui: &mut Ui) {
        // TODO: Add content for the Instruments tab
        ui.label("Instruments Tab");
    }

    fn mixer_tab(&mut self, ui: &mut Ui) {
        // TODO: Add content for the Mixer tab
        ui.label("Mixer Tab");
    }

    fn settings_tab(&mut self, ui: &mut Ui) {
        // TODO: Add content for the Settings tab
        ui.label("Settings Tab");
    }
}
