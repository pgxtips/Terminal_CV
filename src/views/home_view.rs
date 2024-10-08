
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
        CreateBox(42, 6).execute()?;

        // create personal info box content
        execute!(
            std::io::stdout(),
            cursor::MoveTo(4, 4),
            style::Print("Personal"),
            cursor::MoveTo(4, 5),
            style::Print("name: Brandon Lee Gill"),
            cursor::MoveTo(4, 6),
            style::Print("location: Manchester"),
            cursor::MoveTo(4, 7),
            style::Print("email: brandongill123@gmail.com"),
            cursor::MoveTo(4, 8),
            style::Print("link: linkedin.com/in/brandon-lee-gill/"),
            
        )?;

        // education box
        execute!(
            std::io::stdout(),
            cursor::MoveTo(2, 10),
        )?;
        CreateBox(42, 14).execute()?;

        execute!(
            std::io::stdout(),
            cursor::MoveTo(4, 10),
            style::Print("Education"),
            cursor::MoveTo(4, 11),
            style::Print("BSC (Hons) in Computer Science"),
            cursor::MoveTo(4, 12),
            style::Print(" -> Manchester Metropolitan University"),
            cursor::MoveTo(4, 13),
            style::Print(" -> 2021 - 2025"),


            cursor::MoveTo(4, 15),
            style::Print("A-Levels"),
            cursor::MoveTo(4, 16),
            style::Print(" -> Computer Science"),
            cursor::MoveTo(4, 17),
            style::Print(" -> Statistics"),
            cursor::MoveTo(4, 18),
            style::Print(" -> Psychology"),

            cursor::MoveTo(4, 20),
            style::Print("GCSEs"),
            cursor::MoveTo(4, 21),
            style::Print(" -> 9 GCSEs A*-C"),
            cursor::MoveTo(4, 22),
            style::Print(" -> Including English, Maths & Science"),
        )?;

        // work experience box
        execute!(
            std::io::stdout(),
            cursor::MoveTo(45, 4),
        )?;
        CreateBox(58, 20).execute()?;

        execute!(
            std::io::stdout(),
            cursor::MoveTo(47, 4),
            style::Print("Work Experience"),
            cursor::MoveTo(47, 6),
            style::Print("Software Developer - Chippy Digital"),
            cursor::MoveTo(47, 7),
            style::Print(" -> 2023 - Present"),
            cursor::MoveTo(47, 8),
            style::Print(" -> Developed a multitude of web apps and tools for"),
            cursor::MoveTo(47, 9),
            style::Print(" clients in the education education space. "),
            cursor::MoveTo(47, 10),
            style::Print(" Working within a team of 4 engineers, I've developed"),
            cursor::MoveTo(47, 11),
            style::Print(" full stack applications using various technologies "),
            cursor::MoveTo(47, 12),
            style::Print(" and have overseen many projects from start to finish."),
            cursor::MoveTo(47, 13),
            style::Print(" -> Technologies used: HTML, CSS, JavaScript, PHP,"),
            cursor::MoveTo(47, 14),
            style::Print(" Rust, SQL, Postgres, Docker, Github Actions"),
            cursor::MoveTo(47, 15),
            style::Print(" and Digital Ocean."),

            cursor::MoveTo(47, 17),
            style::Print("Bar Staff - Popworld"),
            cursor::MoveTo(47, 18),
            style::Print(" -> 2021 - 2022"),
            cursor::MoveTo(47, 19),
            style::Print(" -> Worked in a diverse team to ensure the prompt"),
            cursor::MoveTo(47, 20),
            style::Print(" delivery of brand standards, as well as excellent"), 
            cursor::MoveTo(47, 21),
            style::Print(" customer service."),
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
            cursor::MoveTo(31, term_size.1 - 2),
            style::SetAttribute(style::Attribute::Bold),
            style::SetForegroundColor(style::Color::Red),
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
            cursor::MoveTo(73, term_size.1 - 2),
            style::Print("3: Projects"),
            cursor::MoveTo(68, term_size.1 - 3),
        )?;
        CreateBox(21, 3).execute()?;

        execute!(
            std::io::stdout(),
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
