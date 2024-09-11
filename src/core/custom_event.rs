use crate::views::view::View;

pub enum CustomEvent{
    ChangeView(Box<dyn View>),
    None,
}
