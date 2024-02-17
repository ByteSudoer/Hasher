use crate::app::state::{Algorithm, AppState};
use crate::constants;
use eframe::egui;

pub fn run_hasher() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT]),
        ..Default::default()
    };
    let mut app = AppState::default();

    eframe::run_simple_native(constants::WINDOW_TITLE, options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(constants::WINDOW_TITLE);
            ui.horizontal(|ui| {
                let name_label = ui.label("The string to hash: ");
                ui.text_edit_singleline(&mut app.input)
                    .labelled_by(name_label.id);
            });
            ui.end_row();
            ui.horizontal(|ui| {
                ui.label("Select a hashing algorithm");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", app.algorithm))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.algorithm, Algorithm::Md5, "MD5");
                        ui.selectable_value(&mut app.algorithm, Algorithm::Md6, "MD6");
                        ui.selectable_value(&mut app.algorithm, Algorithm::Sha1, "SHA-1");
                        ui.selectable_value(&mut app.algorithm, Algorithm::Sha224, "Sha-224");
                        ui.selectable_value(&mut app.algorithm, Algorithm::Sha256, "Sha-256");
                        ui.selectable_value(&mut app.algorithm, Algorithm::Sha512, "SHA-512");
                        ui.selectable_value(&mut app.algorithm, Algorithm::Tiger, "Tiger");
                        ui.selectable_value(&mut app.algorithm, Algorithm::Whirpool, "Whirpool");
                    });
            });
            ui.end_row();
        });
    })
}
