use rusty_taskboard::app::RustyTaskboardApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rusty Taskboards",
        native_options,
        Box::new(|cc| Box::new(RustyTaskboardApp::new(cc))),
    );
}
