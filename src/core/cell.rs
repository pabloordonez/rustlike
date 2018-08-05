use core::color::Color;

pub trait Cell {
    // Sets the cell character.
    fn set_char(&self, character: char);

    // Gets the cell character.
    fn get_char(&self) -> char;

    // Sets the cell colors.
    fn set_colors(&self, foreground: Color, background: Color);

    // sets the background color.
    fn set_bg_color(&self, color: Color);

    // Sets the foreground color.
    fn set_fg_color(&self, color: Color);

    // Gets the background color.
    fn get_bg_color(&self) -> Color;

    // Gets the foreground color.
    fn get_fg_color(&self) -> Color;

    // Copies the cell contents to this refernece.
    fn copy_from(&self, cell: Box<Cell>) {
        self.set_char(cell.get_char());
        self.set_bg_color(cell.get_bg_color());
        self.set_fg_color(cell.get_fg_color());
    }
}
