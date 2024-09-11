use std::io::{stdout, Write};

use crossterm::{cursor, execute, style, terminal};

use crate::{core::custom_event::CustomEvent, utils::create_box::CreateBox, views::view::View};


pub struct AboutMeView{
    has_changed: bool,
}

impl AboutMeView {
    pub fn new() -> Box<AboutMeView> {
        Box::new(AboutMeView{ 
            has_changed: true,
        })
    }
}

impl View for AboutMeView {

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
            cursor::MoveTo(2, 4),
        )?;
        CreateBox(55, 20).execute()?;

        // create personal info box content
        execute!(
            std::io::stdout(),
            cursor::MoveTo(4, 4),
            style::Print("Achievements"),
            cursor::MoveTo(4, 6),
            style::Print("National Citizen Service (NCS)"),
            cursor::MoveTo(4, 7),
            style::Print(" -> Awarded 2018"),
            cursor::MoveTo(4, 8),
            style::Print(" -> This award entailed a 4-week expedition to "),
            cursor::MoveTo(4, 9),
            style::Print(" various places in the UK. This achievement boosts"),
            cursor::MoveTo(4, 10),
            style::Print(" social cohesion,since it is necessary to work with "),
            cursor::MoveTo(4, 11),
            style::Print(" people from a wide variety of backgrounds. It also "),
            cursor::MoveTo(4, 12),
            style::Print(" boosts social mobility and engagement."),

            cursor::MoveTo(4, 14),
            style::Print("Get Involved Volunteering (GIV)"),
            cursor::MoveTo(4, 15),
            style::Print(" -> Awarded 2015"),
            cursor::MoveTo(4, 16),
            style::Print(" -> This project provides the opportunity to work "),
            cursor::MoveTo(4, 17),
            style::Print(" with a group of peers for a charitable cause at "),
            cursor::MoveTo(4, 18),
            style::Print(" Willen Hospice. Successfully completed the task of"),
            cursor::MoveTo(4, 19),
            style::Print(" designing and implementing a garden for children"),
            cursor::MoveTo(4, 20),
            style::Print(" in the hospice to enjoy."),
        )?;

        // create skills and hobbies box
        execute!(
            std::io::stdout(),
            cursor::MoveTo(57, 4),
        )?;
        CreateBox(46, 20).execute()?;

        // create personal info box content
        execute!(
            std::io::stdout(),
            cursor::MoveTo(59, 4),
            style::Print("Skills and Hobbies"),
            cursor::MoveTo(59, 6),
            style::Print("Programming Experience"),
            cursor::MoveTo(59, 7),
            style::Print(" -> HTML, CSS, JavaScript, PHP, Rust, "),
            cursor::MoveTo(59, 8),
            style::Print(" Python, Java, C++, C#, C, Go, SQL, Bash"),
            cursor::MoveTo(59, 10),

            style::Print("Other Techical Skills"),
            cursor::MoveTo(59, 11),
            style::Print(" -> Git, Github Actions, Docker, "),
            cursor::MoveTo(59, 12),
            style::Print(" CI/CD pipeline, Automated Testing, "),
            cursor::MoveTo(59, 13),
            style::Print(" Deployment, Optimisation, SSR/CSR,"),
            cursor::MoveTo(59, 14),
            style::Print(" API, REST, Websockets, Protobuf."),

            cursor::MoveTo(59, 16),
            style::Print("Hobbies"),
            cursor::MoveTo(59, 17),
            style::Print(" -> Programming - since 2013"),
            cursor::MoveTo(59, 18),
            style::Print(" -> Building Computers - since 2014"),
            cursor::MoveTo(59, 19),
            style::Print(" -> Parkour and Skateboarding - since 2015"),
            cursor::MoveTo(59, 20),
            style::Print(" -> Electronics crafts - since 2018"),

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
            style::SetAttribute(style::Attribute::Bold),
            style::SetForegroundColor(style::Color::Red),
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
