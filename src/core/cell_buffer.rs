use core::cell::Cell;
use core::color::Color;
use std::str::Chars;

#[derive(Debug)]
pub struct CellBuffer {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

#[allow(dead_code)]
impl CellBuffer {
    pub fn new(default_cell: Cell, width: usize, height: usize) -> CellBuffer {
        CellBuffer {
            width,
            height,
            cells: vec![default_cell; width * height],
        }
    }

    pub fn resize(&mut self, default_cell: Cell, new_width: usize, new_height: usize) {
        self.width = new_width;
        self.height = new_height;
        self.cells = vec![default_cell;new_width * new_height];
    }

    #[inline]
    pub fn index_of(&self, x: usize, y: usize) -> usize {
        x + self.width * y
    }

    #[inline]
    pub fn coordinates_of(&self, index: usize) -> (usize, usize) {
        if self.width == 0 {
            return (0, 0)
        }

        (index % self.width, index / self.width)
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Cell {
        let index = self.index_of(x, y);
        assert!(index < self.cells.len());
        self.cells[index]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        let index = self.index_of(x, y);
        if index >= self.cells.len() {
            return;
        }
        self.cells[index] = cell;
    }

    pub fn write_chars(&mut self, text: Chars, x: usize, y: usize, foreground: Color, background: Color) {
        let mut index = 0;

        for character in text {
            let buffer_index = self.index_of(x + index, y);

            if buffer_index >= self.cells.len() {
                break;
            }

            self.cells[buffer_index].character = character;
            self.cells[buffer_index].foreground = foreground;
            self.cells[buffer_index].background = background;

            index += 1;
        }
    }

    pub fn write_str(&mut self, text: &str, x: usize, y: usize, foreground: Color, background: Color) {
        self.write_chars(text.chars(), x, y, foreground, background);
    }

    pub fn write_string(&mut self, text: &String, x: usize, y: usize, foreground: Color, background: Color) {
        self.write_chars(text.chars(), x, y, foreground, background);
    }

    pub fn repeat_cell(&mut self, cell: Cell, x: usize, y: usize, length: usize) {
        for index in 0..length {
            let index = self.index_of(x + index, y);

            if index >= self.cells.len() {
                break;
            }

            self.cells[index] = cell;
        }
    }

    pub fn write_cell_buffer(&mut self, cell_buffer: &CellBuffer, x: usize, y: usize) {

        for cby in 0..cell_buffer.height {

            let destination_y = y + cby;

            if destination_y >= self.height {
                break;
            }

            for cbx in 0..cell_buffer.width {
                let destination_x = x + cbx;

                if destination_x >= self.width {
                    break;
                }

                self.set(destination_x, destination_y, cell_buffer.get(cbx, cby));
            }
        }
    }
}
