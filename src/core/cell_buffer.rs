use core::cell::Cell;
use core::point_2d::Point2d;
use core::size_2d::Size2d;
use std::str::Chars;

pub trait CellBuffer {
    fn len(&self) -> usize;

    fn get_cells(&self) -> &Vec<&Cell>;

    fn get_size(&self) -> Size2d;

    fn resize(&mut self, default_cell: &Cell, new_size: Size2d);

    #[inline]
    fn index_of(&self, position: Point2d) -> usize {
        position.x + self.get_size().width * position.y
    }

    #[inline]
    fn coordinates_of(&self, index: usize) -> Point2d {
        if self.get_size().is_empty() || self.get_size().width == 0 || self.get_size().height == 0 {
            return Point2d::empty();
        }

        Point2d::new(index % self.get_size().width, index / self.get_size().width)
    }

    #[inline]
    fn get(&self, position: Point2d) -> &Cell {
        let index = self.index_of(position);
        assert!(index < self.len());
        self.get_cells()[index]
    }

    #[inline]
    fn set(&mut self, position: Point2d, cell: &Cell) {
        let index = self.index_of(position);
        if index >= self.len() {
            return;
        }

        self.get_cells()[index].copy_from(cell);
    }

    fn write_chars(&mut self, text: Chars, position: Point2d, cell: &Cell) {
        let mut index = 0;

        for character in text {
            cell.set_char(character);
            self.set(position.add_x(index), cell);
            index += 1;
        }
    }

    fn write_str(&mut self, text: &str, position: Point2d, cell: &Cell) {
        self.write_chars(text.chars(), position, cell);
    }

    fn write_string(&mut self, text: &String, position: Point2d, cell: &Cell) {
        self.write_chars(text.chars(), position, cell);
    }

    fn repeat_cell(&mut self, cell: &Cell, position: Point2d, length: usize) {
        for index in 0..length {
            self.set(position.add_x(index), cell);
        }
    }

    fn write_cell_buffer(&mut self, cell_buffer: &CellBuffer, position: Point2d) {
        let cell_buffer_size = cell_buffer.get_size();

        for cby in 0..cell_buffer_size.height {
            let destination_y = position.y + cby;

            if destination_y >= self.get_size().height {
                break;
            }

            for cbx in 0..cell_buffer_size.width {
                let destination_x = position.x + cbx;

                if destination_x >= self.get_size().width {
                    break;
                }

                self.set(
                    Point2d::new(destination_x, destination_y),
                    cell_buffer.get(Point2d::new(cbx, cby)),
                );
            }
        }
    }
}
