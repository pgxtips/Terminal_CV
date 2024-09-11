use std::io::stdout;

use crossterm::{cursor, event::{Event, KeyCode}, execute, terminal};

use crate::views::view::View;

use super::custom_event::CustomEvent;

pub struct Application{
    win_title: String,
    win_cols: u16,
    win_rows: u16,
    current_view: Option<Box<dyn View>>,
}


impl Application {

    pub fn new(window_title: String, window_cols: u16, window_rows: u16) -> Application {
        Application {
            win_title: window_title,
            win_cols: window_cols,
            win_rows: window_rows,
            current_view: None,
        }
    }

    fn handle_term_events(&self) {
        let event_stream = crossterm::event::poll(std::time::Duration::from_millis(0));

        if event_stream.unwrap() {
            let cur_event = crossterm::event::read().unwrap();
            match cur_event {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {

                        execute!(
                            stdout(),
                            terminal::Clear(terminal::ClearType::Purge),
                            cursor::Show
                        ).unwrap();

                        terminal::disable_raw_mode().unwrap();

                        std::process::exit(0);
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_custom_events(&mut self, event: CustomEvent) {
        match event {
            CustomEvent::None => {}
            CustomEvent::ChangeView(view) => {
                self.current_view = Some(view);
            }
        }
    }

    pub fn run(&mut self) {

        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            terminal::SetTitle(&self.win_title),
            terminal::SetSize(self.win_cols, self.win_rows)
        ).unwrap();


        // enable raw mode
        terminal::enable_raw_mode().unwrap();

        let splash_view = crate::views::splash_view::SplashView::new();
        self.current_view = Some(splash_view);

        loop {

            self.handle_term_events();

            if self.current_view.is_some() {
                let view = self.current_view.as_mut().unwrap();

                let _ = view.render().unwrap();
                let update_events = view.update().unwrap();
                let input_events = view.handle_input().unwrap();

                self.handle_custom_events(update_events);
                self.handle_custom_events(input_events);
            }

            // run at 30 fps
            std::thread::sleep(std::time::Duration::from_millis(1000 / 120));
        }
    }
}
