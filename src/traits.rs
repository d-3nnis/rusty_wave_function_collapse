use colored::Color;

pub trait AsciiRenderable {
    fn get_ascii_representation(&self) -> char;
}

pub trait ColorRenderable {
    fn get_color(&self) -> Color;
}
