use core::cell::Cell;
use core::cell_buffer::CellBuffer;
use core::size_2d::Size2d;
use windows::Empty;
use windows::cell::NewCharInfo;
use windows::winapi::um::wincon::CHAR_INFO;

pub struct WindowsCellBuffer<'a> {
    pub size: Size2d,
    cells: Vec<&'a CHAR_INFO>,
}

impl<'a> WindowsCellBuffer<'a> {
    pub fn new(default_cell: CHAR_INFO, size: Size2d) -> WindowsCellBuffer<'a> {
        WindowsCellBuffer {
            size,
            cells: vec![&default_cell; size.width * size.height],
        }
    }

    pub fn new_default(size: Size2d) -> WindowsCellBuffer<'a> {
        WindowsCellBuffer {
            size,
            cells: vec![&CHAR_INFO::empty(); size.width * size.height],
        }
    }
}

#[allow(dead_code)]
impl<'a> CellBuffer for WindowsCellBuffer<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.cells.len()
    }

    #[inline]
    pub fn get_cells(&self) -> &Vec {
        &self.cells
    }

    #[inline]
    pub fn get_size(&self) -> Size2d {
        self.size
    }

    pub fn resize(&mut self, default_cell: TCell, new_size: Size2d) {
        self.size = new_size;
        self.cells = vec![default_cell; new_size.width * new_size.height];
    }
}
