use crate::app::state::{hash, Algorithm, AppState};
use crate::constants;
use eframe::egui;

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(constants::WINDOW_TITLE);
            ui.horizontal(|ui| {
                let name_label = ui.label("The string to hash: ");
                ui.text_edit_singleline(&mut self.input)
                    .labelled_by(name_label.id);
            });
            ui.end_row();
            ui.horizontal(|ui| {
                ui.label("Select a hashing algorithm");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.algorithm))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.algorithm, Algorithm::Md5, "MD5");
                        ui.selectable_value(&mut self.algorithm, Algorithm::Md6, "MD6");
                        ui.selectable_value(&mut self.algorithm, Algorithm::Sha1, "SHA-1");
                        ui.selectable_value(&mut self.algorithm, Algorithm::Sha224, "Sha-224");
                        ui.selectable_value(&mut self.algorithm, Algorithm::Sha256, "Sha-256");
                        ui.selectable_value(&mut self.algorithm, Algorithm::Sha512, "SHA-512");
                        ui.selectable_value(&mut self.algorithm, Algorithm::Tiger, "Tiger");
                        ui.selectable_value(&mut self.algorithm, Algorithm::Whirpool, "Whirpool");
                    });
                if ui.add(egui::widgets::Button::new("Hash me")).clicked() {
                    self.result = hash(&self.input, self.algorithm);
                }
            });
            ui.end_row();
            ui.horizontal(|ui| {
                let result_label = ui.label("Hash Result :");

                ui.text_edit_multiline(&mut self.result)
                    .labelled_by(result_label.id);
            });
        });
    }
}

pub fn run_hasher() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT]),
        ..Default::default()
    };
    eframe::run_native(
        constants::WINDOW_TITLE,
        options,
        Box::new(|_cc| Box::<AppState>::default()),
    )

    // eframe::run_simple_native(constants::WINDOW_TITLE, options, move |ctx, _frame| {})
}
