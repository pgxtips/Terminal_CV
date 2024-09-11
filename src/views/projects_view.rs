use std::io::{stdout, Write};

use crossterm::{cursor, execute, style, terminal};

use crate::{core::custom_event::CustomEvent, utils::create_box::CreateBox, views::view::View};


pub struct ProjectsView{
    has_changed: bool,
}

impl ProjectsView {
    pub fn new() -> Box<ProjectsView> {
        Box::new(ProjectsView{ 
            has_changed: true,
        })
    }
}

impl View for ProjectsView {

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


        // create achievements box
        execute!(
            std::io::stdout(),
            cursor::MoveTo(2, 3),
        )?;

        CreateBox(term_size.0 - 4, 21).execute()?;

        // create personal info box content
        execute!(
            std::io::stdout(),
            cursor::MoveTo(4, 3),
            style::Print("Projects"),
            cursor::MoveTo(4, 4),
            style::Print("GitHub: github.com/pgxtips/"),

            cursor::MoveTo(4, 6),
            style::Print("The Binding of Isaac Achievement Tracker"),
            cursor::MoveTo(4, 7),
            style::Print(" -> github.com/pgxtips/tboiSecrets"),
            cursor::MoveTo(4, 8),
            style::Print(" -> https://www.tboisecrets.com"),
            cursor::MoveTo(4, 9),
            style::Print(" -> An achievement tracker for the game The Binding of Isaac, it uses"),
            cursor::MoveTo(4, 10),
            style::Print(" the Steam API to get all of the user's steam data related to the game"),
            cursor::MoveTo(4, 11),
            style::Print(" and displays them in the style of the game"),

            cursor::MoveTo(4, 13),
            style::Print("Terminal CV"),
            cursor::MoveTo(4, 14),
            style::Print(" -> github.com/pgxtips/Terminal_CV"),
            cursor::MoveTo(4, 15),
            style::Print(" -> A terminal based CV written in Rust using the crossterm library. "),
            cursor::MoveTo(4, 16),
            style::Print(" It is a simple CV that can be navigated using the keyboard and "),
            cursor::MoveTo(4, 17),
            style::Print(" is accessed via ssh. I have enjoyed working on this, however, I "),
            cursor::MoveTo(4, 18),
            style::Print(" used a rust library called crossterm since it has terminal bindings for "),
            cursor::MoveTo(4, 19),
            style::Print(" Windows. Window uses the WIN32 API for IO, instead of stty like found"),
            cursor::MoveTo(4, 20),
            style::Print(" on Unix systems. I did not want to create this myself since it would be "),
            cursor::MoveTo(4, 21),
            style::Print(" a lot of work."),
        )?;

        // footer
        execute!(
            std::io::stdout(),
            cursor::MoveTo(4, term_size.1 - 2),
            style::Print("Press 'q' to quit"),
            cursor::MoveTo(2, term_size.1 - 3),
        )?;
        CreateBox(21, 3).execute()?;

        execute!(
            std::io::stdout(),
            style::SetAttribute(style::Attribute::Reset),
            cursor::MoveTo(31, term_size.1 - 2),
            style::Print("1: Home"),
            cursor::MoveTo(24, term_size.1 - 3),
        )?;
        CreateBox(21, 3).execute()?;

        execute!(
            std::io::stdout(),
            style::SetAttribute(style::Attribute::Reset),
            cursor::MoveTo(51, term_size.1 - 2),
            style::Print("2: About Me"),
            cursor::MoveTo(46, term_size.1 - 3),
        )?;
        CreateBox(21, 3).execute()?;

        execute!(
            std::io::stdout(),
            style::SetAttribute(style::Attribute::Reset),
            style::SetAttribute(style::Attribute::Bold),
            style::SetForegroundColor(style::Color::Red),
            cursor::MoveTo(73, term_size.1 - 2),
            style::Print("3: Projects"),
            cursor::MoveTo(68, term_size.1 - 3),
        )?;
        CreateBox(21, 3).execute()?;

        execute!(
            std::io::stdout(),
            style::SetAttribute(style::Attribute::Reset),
            cursor::MoveTo(90, term_size.1 - 3),
            style::Print("use numbers to"),
            cursor::MoveTo(90, term_size.1 - 2),
            style::Print("navigate the "),
            cursor::MoveTo(90, term_size.1 - 1),
            style::Print("different pages"),
        )?;


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
