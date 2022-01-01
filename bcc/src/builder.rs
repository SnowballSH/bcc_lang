use crate::model::{Cells, MemoryModel};
use crate::optimizer::optimize;

#[derive(Clone, Debug, Default)]
pub struct Builder {
    pub(crate) model: MemoryModel,
    pub cursor: usize,
    pub result: Vec<u8>,
}

impl Builder {
    /*
        Allocating
     */

    pub fn n_cells(&mut self, size: usize) -> Cells {
        let cell = self.model.allocate(size);
        self.goto(cell.position);
        cell
    }

    pub fn new_cell(&mut self) -> &mut Self {
        let cell = self.model.allocate(1);
        self.goto(cell.position);
        self
    }

    pub fn free_last_n(&mut self, size: usize) -> &mut Self {
        self.model.free(size);
        for _ in 0..size {
            self.push_str("[-]");
            self.back();
        }
        self
    }

    /*
        Positioning
     */

    pub fn goto(&mut self, index: usize) -> &mut Self {
        let diff = index as isize - self.cursor as isize;
        if diff > 0 {
            for _ in 0..diff {
                self.result.push('>' as u8);
            }
        } else if diff < 0 {
            for _ in 0..-diff {
                self.result.push('<' as u8);
            }
        }
        self.cursor = index;
        self
    }

    pub fn advance(&mut self) -> &mut Self {
        self.cursor += 1;
        self.result.push('>' as u8);
        self
    }

    pub fn back(&mut self) -> &mut Self {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.result.push('<' as u8);
        }
        self
    }

    #[inline]
    fn push_str(&mut self, s: &str) {
        self.result.extend_from_slice(s.as_bytes());
    }

    /*
        Writing/Clearing
     */

    pub fn clear(&mut self) {
        self.push_str("[-]");
    }

    pub fn override_one_byte(&mut self, byte: u8) -> &mut Self {
        self.clear();
        self.add(byte);
        self
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> Cells {
        let cells = self.n_cells(bytes.len());
        for byte in bytes {
            self.override_one_byte(*byte);
            self.advance();
        }
        cells
    }

    pub fn write_bytes_unchecked(&mut self, bytes: &[u8]) -> Cells {
        let cells = self.n_cells(bytes.len());
        for byte in bytes {
            self.add(*byte);
            self.advance();
        }
        cells
    }

    /*
        Arithmetic
     */

    pub fn add(&mut self, amount: u8) -> &mut Self {
        for _ in 0..amount {
            self.result.push('+' as u8);
        }
        self
    }

    pub fn sub(&mut self, amount: u8) -> &mut Self {
        for _ in 0..amount {
            self.result.push('-' as u8);
        }
        self
    }

    pub fn adjust(&mut self, amount: i16) -> &mut Self {
        if amount > 0 {
            self.add(amount as u8);
        } else {
            self.sub(-amount as u8);
        }
        self
    }

    /*
        Algorithms
        https://esolangs.org/wiki/Brainfuck_algorithms
     */

    pub fn copy(&mut self, source: usize, target: usize) -> &mut Self {
        let temp = self.n_cells(1).position;
        self.goto(target).clear();
        self.goto(source);

        self.result.push('[' as u8);
        self.goto(target);
        self.result.push('+' as u8);
        self.goto(temp);
        self.result.push('+' as u8);
        self.goto(source);
        self.result.push('-' as u8);
        self.result.push(']' as u8);

        self.goto(temp);
        self.result.push('[' as u8);
        self.goto(source);
        self.result.push('+' as u8);
        self.goto(temp);
        self.result.push('-' as u8);
        self.result.push(']' as u8);

        self.free_last_n(1);

        self
    }

    /*
        I/O
     */

    pub fn print_byte(&mut self) -> &mut Self {
        self.result.push('.' as u8);
        self
    }

    pub fn print_cells(&mut self, cells: Cells) -> &mut Self {
        let orig = cells.position;
        self.goto(orig);
        for _ in 0..cells.size {
            self.result.push('.' as u8);
            self.advance();
        }
        self
    }

    pub fn just_print(&mut self, string: &str) -> &mut Self {
        let c = self.n_cells(1);
        self.goto(c.position);
        let mut ascii = vec![0i16];
        for c in string.chars() {
            ascii.push(c as i16);
        }
        for (i, ch) in ascii.iter().enumerate().skip(1) {
            self.adjust(ch - ascii[i - 1]).print_byte();
        }
        self.free_last_n(1);
        self
    }

    pub fn input_byte(&mut self) -> &mut Self {
        self.result.push(',' as u8);
        self
    }

    /*
        Finishing
     */

    pub fn finish(&mut self) -> &str {
        optimize(&mut self.result);
        unsafe {
            std::str::from_utf8_unchecked(&*self.result)
        }
    }
}
