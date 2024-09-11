use crate::core::custom_event::CustomEvent;

pub trait View{
    fn render(&mut self) -> Result<(), std::io::Error>;
    fn update(&self) -> Result<CustomEvent, std::io::Error>;
    fn handle_input(&self) -> Result<CustomEvent, std::io::Error>;
}
