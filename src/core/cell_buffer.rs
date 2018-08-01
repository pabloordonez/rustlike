use core::cell::Cell;
use core::point_2d::Point2d;
use core::size_2d::Size2d;
use std::slice::Iter;
use std::str::Chars;

#[derive(Debug)]
pub struct CellBuffer<TCell> {
    pub size: Size2d,
    cells: Vec<TCell>,
}

#[allow(dead_code)]
impl<TCell> CellBuffer<TCell>
where
    TCell: Cell,
{
    pub fn new(default_cell: TCell, size: Size2d) -> CellBuffer<TCell> {
        CellBuffer {
            size,
            cells: vec![default_cell; size.width * size.height],
        }
    }

    pub fn iter(&self) -> Iter<'_, TCell> {
        self.cells.iter()
    }

    pub fn resize(&mut self, default_cell: TCell, new_size: Size2d) {
        self.size = new_size;
        self.cells = vec![default_cell; new_size.width * new_size.height];
    }

    #[inline]
    pub fn index_of(&self, position: Point2d) -> usize {
        position.x + self.size.width * position.y
    }

    #[inline]
    pub fn coordinates_of(&self, index: usize) -> Point2d {
        if self.size.is_empty() || self.size.width == 0 || self.size.height == 0 {
            return Point2d::empty();
        }

        Point2d::new(index % self.size.width, index / self.size.width)
    }

    #[inline]
    pub fn get(&self, position: Point2d) -> TCell {
        let index = self.index_of(position);
        assert!(index < self.cells.len());
        self.cells[index]
    }

    #[inline]
    pub fn set(&mut self, position: Point2d, cell: TCell) {
        let index = self.index_of(position);
        if index >= self.cells.len() {
            return;
        }
        self.cells[index] = cell;
    }

    pub fn write_chars(&mut self, text: Chars, position: Point2d, cell: TCell) {
        let mut index = 0;

        for character in text {
            let buffer_index = self.index_of(position.add_x(index));

            if buffer_index >= self.cells.len() {
                break;
            }

            cell.set_char(character);
            self.cells[buffer_index] = cell;

            index += 1;
        }
    }

    pub fn write_str(&mut self, text: &str, position: Point2d, cell: TCell) {
        self.write_chars(text.chars(), position, cell);
    }

    pub fn write_string(&mut self, text: &String, position: Point2d, cell: TCell) {
        self.write_chars(text.chars(), position, cell);
    }

    pub fn repeat_cell(&mut self, cell: TCell, position: Point2d, length: usize) {
        for index in 0..length {
            let buffer_index = self.index_of(position.add_x(index));

            if buffer_index >= self.cells.len() {
                break;
            }

            self.cells[buffer_index] = cell;
        }
    }

    pub fn write_cell_buffer(&mut self, cell_buffer: &CellBuffer<TCell>, position: Point2d) {
        for cby in 0..cell_buffer.size.height {
            let destination_y = position.y + cby;

            if destination_y >= self.size.height {
                break;
            }

            for cbx in 0..cell_buffer.size.width {
                let destination_x = position.x + cbx;

                if destination_x >= self.size.width {
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
