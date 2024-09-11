
use std::io::{stdout, Write};

use crossterm::{cursor, execute, style, terminal};

use crate::{core::custom_event::CustomEvent, utils::create_box::CreateBox, views::view::View};


pub struct HomeView{
    has_changed: bool,
}

impl HomeView {
    pub fn new() -> Box<HomeView> {
        Box::new(HomeView{ 
            has_changed: true,
        })
    }
}

impl View for HomeView {

    fn render(&mut self) -> Result<(), std::io::Error> {

        if !self.has_changed { return Ok(()); }

        // init render
        execute!(
            std::io::stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::Hide,
        )?;

        execute!(
            std::io::stdout(),
            cursor::MoveTo(0, 0),
        )?;

        let term_size = terminal::size()?;

        // create header box 
        CreateBox(term_size.0, 3).execute()?;
        let header_text = "Brandon Lee Gill - Terminal CV";
        let header_middle_rows = (term_size.0 / 2) - (header_text.len() as u16 / 2);
        execute!(
            std::io::stdout(),
            cursor::MoveTo(header_middle_rows, 1),
            style::Print(header_text),
        )?;


        // create personal info box
        execute!(
            std::io::stdout(),
            cursor::MoveTo(2, 4),
        )?;
        CreateBox(35, 5).execute()?;

        // create personal info box content
        execute!(
            std::io::stdout(),
            cursor::MoveTo(4, 4),
            style::Print("Personal"),
            cursor::MoveTo(4, 5),
            style::Print("name: Brandon Lee Gill"),
            cursor::MoveTo(4, 6),
            style::Print("email: brandongill123@gmail.com"),
            cursor::MoveTo(4, 7),
            style::Print("location: Manchester"),
        )?;

        // education box
        execute!(
            std::io::stdout(),
            cursor::MoveTo(2, 10),
        )?;
        CreateBox(40, 14).execute()?;

        execute!(
            std::io::stdout(),
            cursor::MoveTo(3, 10),
            style::Print("Education"),
            cursor::MoveTo(3, 11),
            style::Print("BSC (Hons) in Computer Science"),
            cursor::MoveTo(3, 12),
            style::Print(" -> Manchester Metropolitan University"),
            cursor::MoveTo(3, 13),
            style::Print(" -> 2021 - 2025"),


            cursor::MoveTo(3, 15),
            style::Print("A-Levels"),
            cursor::MoveTo(3, 16),
            style::Print(" -> Computer Science"),
            cursor::MoveTo(3, 17),
            style::Print(" -> Statistics"),
            cursor::MoveTo(3, 18),
            style::Print(" -> Psychology"),

            cursor::MoveTo(3, 20),
            style::Print("GCSEs"),
            cursor::MoveTo(3, 21),
            style::Print(" -> 9 GCSEs A-C"),
            cursor::MoveTo(3, 22),
            style::Print(" -> Including Maths, English & Science"),


        )?;


        // footer
        let footer_text = "Press 'q' to quit";
        let footer_middle_col = (term_size.0 / 2) - (footer_text.len() as u16 / 2);
        
        execute!(
            std::io::stdout(),
            cursor::MoveTo(4, term_size.1 - 2),
            style::Print("Press 'q' to quit"),
            cursor::MoveTo(2, term_size.1 - 3),
        )?;
        CreateBox(21, 3).execute()?;


        self.has_changed = false;
        Ok(())
    }

    fn update(&self) -> Result<CustomEvent, std::io::Error>{
        Ok(CustomEvent::None)
    }

    fn handle_input(&self) -> Result<CustomEvent, std::io::Error> {
        Ok(CustomEvent::None)
    }
}
