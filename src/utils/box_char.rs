pub enum BoxChar {
    HorizontalLine,
    HorizontalLineBold,
    VerticalLine,
    TopLeftBoxCorner, 
    TopRightBoxCorner, 
    BotLeftBoxCorner,
    BotRightBoxCorner,
}

impl BoxChar {
    pub fn char(&self) -> Vec<u8> {
        match self {
            Self::HorizontalLine => vec![0xE2, 0x94, 0x80],
            Self::HorizontalLineBold => vec![0xE2, 0x94, 0x81],
            Self::VerticalLine => vec![0xE2, 0x94, 0x82],
            Self::TopLeftBoxCorner => vec![0xE2, 0x94, 0x8C],
            Self::TopRightBoxCorner => vec![0xE2, 0x94, 0x90],
            Self::BotLeftBoxCorner => vec![0xE2, 0x94, 0x94],
            Self::BotRightBoxCorner => vec![0xE2, 0x94, 0x98],
        }
    }
}
