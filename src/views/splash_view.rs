
use crossterm::{cursor, execute, style, terminal};

use crate::{core::custom_event::CustomEvent, views::view::View};

pub struct SplashView{
    has_changed: bool,
    start_time: std::time::Instant,
}

impl SplashView {
    pub fn new() -> Box<SplashView> {
        Box::new(SplashView {
            has_changed: true,
            start_time: std::time::Instant::now(),
        })
    }
}

impl View for SplashView {

    fn render(&mut self) -> Result<(), std::io::Error> {

        if !self.has_changed { return Ok(()); }

        let window_size = crossterm::terminal::size()?;

        let splash_text = r#"Terminal CV"#;

        let splash_text_len = splash_text.len();

        let x = (window_size.0 / 2) - (splash_text_len as u16 / 2);
        let y = window_size.1 / 2;

        execute!(
            std::io::stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::Hide,
            cursor::MoveTo(x, y),
            style::Print(splash_text)
        )?;

        self.has_changed = false;
        Ok(())
    }

    fn update(&self) -> Result<CustomEvent, std::io::Error>{
        let current_time = std::time::Instant::now();
        let elapsed_time = current_time.duration_since(self.start_time);

        if elapsed_time.as_secs() > 1 {
            let home_view = crate::views::home_view::HomeView::new();
            return Ok(CustomEvent::ChangeView(home_view));
        }

        Ok(CustomEvent::None)
    }

    fn handle_input(&self) -> Result<CustomEvent, std::io::Error> {
        Ok(CustomEvent::None)
    }
}
