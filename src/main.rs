use rusty_taskboard::app::RustyTaskboardApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    match eframe::run_native(
        "Rusty Taskboards",
        native_options,
        Box::new(|cc| Box::new(RustyTaskboardApp::new(cc))),
    ) {
        Ok(()) => (),
        Err(error) => eprintln!("An error has occured! {}", error),
    }
}
