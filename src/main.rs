use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Mandelbrot Set Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(MbsApp::default()))),
    )
}

#[derive(Default)]
struct MbsApp;

impl eframe::App for MbsApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.label("Hello, Mandelbrot!");
    }
}