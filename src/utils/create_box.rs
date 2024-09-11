use crossterm::{cursor, execute, style};

use super::box_char::BoxChar;

/// CreateBox is a command that creates a box of a given width and height
/// width and height are (columns, rows)
pub struct CreateBox(pub u16, pub u16);

impl CreateBox {
    pub fn execute(&self) -> Result<(), std::io::Error>{

        let (width, height) = (self.0, self.1);
        let (cursor_col, cursor_row) = crossterm::cursor::position()?;

        // top border
        let top_line: String  = {
            let mut line = String::new();
            line += &String::from_utf8(BoxChar::TopLeftBoxCorner.char()).unwrap();

            for _ in 0..width-2 {
                let char = String::from_utf8(BoxChar::HorizontalLine.char()).unwrap();
                line += &char;
            }
            line += &String::from_utf8(BoxChar::TopRightBoxCorner.char()).unwrap();
            line 
        }; 

        // bottom border
        let bottom_line: String  = {
            let mut line = String::new();
            line += &String::from_utf8(BoxChar::BotLeftBoxCorner.char()).unwrap();

            for _ in 0..width-2 {
                let char = String::from_utf8(BoxChar::HorizontalLine.char()).unwrap();
                line += &char;
            }
            line += &String::from_utf8(BoxChar::BotRightBoxCorner.char()).unwrap();
            line 
        }; 

        // write top border and bottom border
        execute!(
            std::io::stdout(),
            cursor::MoveTo(cursor_col, cursor_row),
            style::Print(top_line),
            cursor::MoveTo(cursor_col, cursor_row + height - 1),
            style::Print(bottom_line),
        )?;

        for idx in 0..height-2 {
            // write left and right border
            execute!(
                std::io::stdout(),
                cursor::MoveTo(cursor_col, cursor_row + idx + 1),
                style::Print(String::from_utf8(BoxChar::VerticalLine.char()).unwrap()),
                cursor::MoveTo(cursor_col + width - 1, cursor_row + idx + 1),
                style::Print(String::from_utf8(BoxChar::VerticalLine.char()).unwrap()),
            )?;
        }


        Ok(())
    }
}
