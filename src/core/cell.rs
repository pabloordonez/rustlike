use core::color::Color;

pub trait Cell {
    // Sets the cell character.
    fn set_char(&self, character: char);

    // Sets the cell colors.
    fn set_colors(foreground: Color, background: Color);

    // sets the background color.
    fn set_bg_color(&self, color: Color);

    // Sets the foreground color.
    fn set_fg_color(&self, color: Color);

    // Gets the background color.
    fn get_bg_color(&self) -> Color;

    // Gets the foreground color.
    fn get_fg_color(&self) -> Color;
}
