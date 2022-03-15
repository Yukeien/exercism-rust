use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Clone, Copy, Debug, IntoEnumIterator, Eq, PartialEq, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9
}

impl ResistorColor {
    fn as_str(&self) -> &'static str {
        match self {
            ResistorColor::Black => "Black",
            ResistorColor::Brown => "Brown",
            ResistorColor::Red => "Red",
            ResistorColor::Orange => "Orange",
            ResistorColor::Yellow => "Yellow",
            ResistorColor::Green => "Green",
            ResistorColor::Blue => "Blue",
            ResistorColor::Violet => "Violet",
            ResistorColor::Grey => "Grey",
            ResistorColor::White => "White",
        }
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    let resistor_color = ResistorColor::into_enum_iter().find(|item| item.int_value() == value);

    if resistor_color == None {
        return "value out of range".to_string();
    }

    resistor_color.unwrap().as_str().to_string()
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
