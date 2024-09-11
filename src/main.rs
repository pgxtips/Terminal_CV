use core::application::Application;

pub mod core;
pub mod views;
pub mod utils;

fn main() -> std::io::Result<()> {

    let win_cols = 80;
    let win_rows = 24;
    let title = String::from("Terminal CV - Brandon Gill");

    let mut application = Application::new(title, win_cols, win_rows);
    application.run(); 

    Ok(())
}
