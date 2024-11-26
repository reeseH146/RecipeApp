// References other modules in this file
mod recipe;
mod manager;
mod ui;

// Runs the ui module with the result being the running of the iced package
fn main() -> iced::Result {
    ui::run()
}