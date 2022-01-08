use std::collections::BTreeMap;

use crate::model::{Cells, MemoryModel};
use crate::optimizer::optimize;

#[derive(Clone, Debug)]
pub struct Builder {
    pub(crate) model: MemoryModel,
    pub cursor: usize,
    pub result: Vec<u8>,
    pub constants: BTreeMap<u8, usize>,
}

impl Builder {
    pub fn new() -> Self {
        let mut s = Self {
            model: MemoryModel::new(),
            cursor: 0,
            result: Vec::with_capacity(64),
            constants: BTreeMap::new(),
        };

        s.new_constant(0);

        s
    }

    /*
        Allocating
     */

    pub fn new_constant(&mut self, value: u8) -> &mut Self {
        let cell = self.n_cells(1);
        self.goto(cell.position).add(value);
        self.constants.insert(value, cell.position);
        self
    }

    pub fn n_cells(&mut self, size: usize) -> Cells {
        let cell = self.model.allocate(size);
        self.goto(cell.position);
        cell
    }

    pub fn new_byte(&mut self, n: u8) -> Cells {
        let cell = self.model.allocate(1);
        self.goto(cell.position);
        self.add(n);
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

    pub fn clear(&mut self) -> &mut Self {
        self.push_str("[-]");
        self
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
        Arithmetic Algorithms
     */

    /// Consume current cell and add to pos
    pub fn add_to(&mut self, pos: usize) -> &mut Self {
        let start = self.cursor;
        self
            .start_while()
            .goto(pos)
            .add(1)
            .goto(start)
            .sub(1)
            .end_while_unchecked()
    }

    /// Consume current cell and sub from pos
    pub fn sub_from(&mut self, pos: usize) -> &mut Self {
        let start = self.cursor;
        self
            .start_while()
            .goto(pos)
            .sub(1)
            .goto(start)
            .sub(1)
            .end_while_unchecked()
    }

    /// Consume current cell and mul to pos
    pub fn mul_to(&mut self, pos: usize) -> &mut Self {
        let start = self.cursor;
        let original = self.n_cells(1);
        self.copy(pos, original.position);
        let temp = self.n_cells(1);
        self
            .goto(pos)
            .clear()
            .goto(start)
            .start_while()
            .copy(original.position, temp.position)
            .goto(temp.position)
            .add_to(pos)
            .goto(start)
            .sub(1)
            .end_while_unchecked();

        self.goto(temp.position).free_last_n(1);
        self.goto(original.position).free_last_n(1);
        self
    }

    /// Consume current cell and current cell + 1. Current cell becomes quotient.
    pub fn div_by(&mut self) -> &mut Self {
        // https://esolangs.org/wiki/Brainfuck_algorithms#Divmod_algorithm
        // input >n d
        self.push_str("[->-[>+>>]>[+[-<+>]>+>>]<<<<<]");
        // output >0 d-n%d n%d n/d

        let p1 = self.cursor;
        let p2 = self.cursor + 1;
        let rem = self.cursor + 2;
        let quo = self.cursor + 3;

        self.goto(quo).add_to(p1);
        self.goto(p2).clear().goto(rem).clear();

        self.goto(p1)
    }

    /// Consume current cell and current cell + 1. Current cell becomes remainder.
    pub fn mod_by(&mut self) -> &mut Self {
        // https://esolangs.org/wiki/Brainfuck_algorithms#Divmod_algorithm
        // input >n d
        self.push_str("[->-[>+>>]>[+[-<+>]>+>>]<<<<<]");
        // output >0 d-n%d n%d n/d

        let p1 = self.cursor;
        let p2 = self.cursor + 1;
        let rem = self.cursor + 2;
        let quo = self.cursor + 3;

        self.goto(rem).add_to(p1);
        self.goto(p2).clear().goto(quo).clear();

        self.goto(p1)
    }

    /// Consume current cell and current cell + 1. Current cell becomes quotient and pos becomes remainder
    pub fn divmod_by(&mut self) -> &mut Self {
        // https://esolangs.org/wiki/Brainfuck_algorithms#Divmod_algorithm
        // input >n d
        self.push_str("[->-[>+>>]>[+[-<+>]>+>>]<<<<<]");
        // output >0 d-n%d n%d n/d

        let p1 = self.cursor;
        let p2 = self.cursor + 1;
        let rem = self.cursor + 2;
        let quo = self.cursor + 3;

        self.goto(quo).add_to(p1);
        self.goto(p2).clear().goto(rem).add_to(p2);

        self.goto(p1)
    }

    /*
        Algorithms
        https://esolangs.org/wiki/Brainfuck_algorithms
     */

    pub fn copy(&mut self, source: usize, target: usize) -> &mut Self {
        let temp = self.n_cells(1).position;
        self.goto(target).clear();
        self.goto(source);

        self.start_while()
            .goto(target)
            .add(1)
            .goto(temp)
            .add(1)
            .goto(source)
            .sub(1)
            .end_while_unchecked();

        self.goto(temp);
        self.start_while()
            .goto(source)
            .add(1)
            .goto(temp)
            .sub(1)
            .end_while_unchecked();

        self.goto(temp);

        self.free_last_n(1);

        self
    }

    pub fn start_if(&mut self) -> &mut Self {
        self.result.push('[' as u8);
        self
    }

    pub fn end_if(&mut self) -> &mut Self {
        self.clear();
        self.result.push(']' as u8);
        self
    }

    pub fn start_while(&mut self) -> &mut Self {
        self.result.push('[' as u8);
        self
    }

    pub fn end_while(&mut self, pos: usize) -> &mut Self {
        self.goto(pos).result.push(']' as u8);
        self
    }

    pub fn end_while_unchecked(&mut self) -> &mut Self {
        self.result.push(']' as u8);
        self
    }

    /*
        I/O
     */

    pub fn print_ascii(&mut self) -> &mut Self {
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

    pub fn print_as_byte(&mut self) -> &mut Self {
        // http://stackoverflow.com/questions/12569444/printing-a-number-in-brainfuck
        self.push_str(">>++++++++++<<[->+>-[>+>>]>[+[-<+>]>+>>]<<<<<<]>>[-]>>>++++++++++<[->-[>+>>]>[+[-<+>]>+>>]<<<<<]>[-]>>[>++++++[-<++++++++>]<.<<+>+>[-]]<[<[->-<]++++++[->++++++++<]>.[-]]<<++++++[-<++++++++>]<.[-]<<[-<+>]<");

        // already should go to original cell and leave everything as 0

        self
    }

    pub fn just_print(&mut self, string: &str) -> &mut Self {
        let c = self.n_cells(1);
        self.goto(c.position);
        self.just_print_here(string);
        self.free_last_n(1);
        self
    }

    pub fn just_print_here(&mut self, string: &str) -> &mut Self {
        let mut ascii = vec![0i16];
        for c in string.chars() {
            ascii.push(c as i16);
        }
        for (i, ch) in ascii.iter().enumerate().skip(1) {
            self.adjust(ch - ascii[i - 1]).print_ascii();
        }
        self.clear();
        self
    }

    pub fn input_byte(&mut self) -> &mut Self {
        self.result.push(',' as u8);
        self
    }

    /*
        Finishing
     */

    pub fn finish(&mut self, do_optimize: bool) -> &str {
        if do_optimize {
            optimize(&mut self.result);
        }
        unsafe {
            std::str::from_utf8_unchecked(&*self.result)
        }
    }
}
