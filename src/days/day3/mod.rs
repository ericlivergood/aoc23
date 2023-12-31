use std::ops::Range;
use array2d::Array2D;
use crate::common::input_reader;

mod tests;
pub struct Day;

struct Schematic {
    data: Array2D<char>
}

#[derive(Clone)]
struct PartNumber {
    y: usize,
    x1: usize,
    x2: usize,
    value: String
}

impl PartNumber {
    pub fn as_number(&self) -> u32 {
        self.value.parse::<u32>()
            .expect("could not parse number.")
    }

    pub fn x_adjacency_range(&self, max_x: usize) -> Range<usize> {
        let start = if self.x1 == 0 { 0 } else { self.x1 - 1 };
        let end = if self.x2 == max_x { self.x2 } else { self.x2 + 1 };
        start..end
    }
}

fn is_symbol(c: &char) -> bool {
    if c.is_digit(10) {
        return false;
    }
    match c {
        '.' => false,
        _=> true
    }
}


impl Schematic {
    pub fn is_number_at(&self, x: usize, y: usize) -> bool {
        let val = self.data.get(y, x)
            .expect("unable to get element @ {x},{y}");

        val.is_digit(10)
    }
    pub fn is_symbol_at(&self, x: usize, y: usize) -> bool {
        let val = self.data.get(y, x)
            .expect("unable to get element");
        is_symbol(val)
    }

    pub fn get_part_numbers_adjacent_to(&self, x: usize, y: usize) -> Vec<PartNumber> {
        let mut adjacents = Vec::new();
        for part in self.get_part_numbers() {
            let x_range = part.x_adjacency_range(self.data.row_len());
            if (y > 0 && part.y == y - 1) || (part.y == y + 1) || (part.y == y) {
                if x >= x_range.start && x <= x_range.end {
                    adjacents.push(part);
                }
            }
        }
        adjacents
    }

    pub fn is_gear(&self, x: usize, y: usize) -> bool {
        let val = self.data.get(y, x)
            .expect("unable to get element");
        if val != &'*' {
            return false;
        }

        let adjacent_parts = self.get_part_numbers_adjacent_to(x, y);
        adjacent_parts.len() == 2
    }

    pub fn is_adjacent_to_gear(&self, p: &PartNumber) -> bool {
        let start_x = if p.x1 == 0 { 0 } else { p.x1 -1 };
        let end_x = if p.x2 == self.data.row_len() - 1 { p.x2 } else { p.x2 + 1 };

        if self.is_gear(start_x, p.y) {
            return true;
        }
        if self.is_gear(end_x, p.y) {
            return true;
        }

        if p.y > 0 {
            for x in start_x..end_x+1 {
                let y = p.y - 1;
                if self.is_gear(x, y) {
                    return true;
                }
            }
        }

        if p.y < self.data.column_len() - 1 {
            for x in start_x..end_x+1 {
                let y = p.y + 1;
                if self.is_gear(x, y) {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_adjacent_to_symbol(&self, p: &PartNumber) -> bool {
        let start_x = if p.x1 == 0 { 0 } else { p.x1 -1 };
        let end_x = if p.x2 == self.data.row_len() - 1 { p.x2 } else { p.x2 + 1 };

        if self.is_symbol_at(start_x, p.y) {
            return true;
        }
        if self.is_symbol_at(end_x, p.y) {
            return true;
        }

        if p.y > 0 {
            for x in start_x..end_x+1 {
                let y = p.y - 1;
                if self.is_symbol_at(x, y) {
                    return true;
                }
            }
        }

        if p.y < self.data.column_len() - 1 {
            for x in start_x..end_x+1 {
                let y = p.y + 1;
                if self.is_symbol_at(x, y) {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_part_numbers(&self) -> Vec<PartNumber> {
        let mut part_numbers = Vec::new();
        let mut current_part_number = PartNumber {
            x1: 0,
            x2: 0,
            y: 0,
            value: String::from("")
        };
        let mut y = 0;
        for row in &self.data.as_rows() {
            let mut x: usize = 0;
            let mut last_was_number = false;
            for c in row {
                if c.is_digit(10) {
                    if last_was_number {
                        current_part_number.x2 = x;
                        current_part_number.value.push(*c);
                    }
                    else {
                        last_was_number = true;
                        current_part_number = PartNumber {
                            x1: x,
                            x2: x,
                            y,
                            value: String::from(*c)
                        };
                    }
                }
                else {
                    if last_was_number {
                        part_numbers.push(current_part_number.clone());
                    }
                    last_was_number = false;
                }
                x += 1;
            }
            if last_was_number {
                part_numbers.push(current_part_number.clone());
            }
            y += 1;
        }

        part_numbers
    }
}

impl Day {
    pub fn run(&self) {
        self.run_part_two();
    }
    fn run_part_one(&self) {
        let r = input_reader::InputReader;
        let data = r.get_as_2d_array("/git/aoc23/src/days/day3/input");
        let schematic = Schematic {
            data
        };
        let parts = schematic.get_part_numbers();
        let mut sum = 0;
        for p in parts {
            if schematic.is_adjacent_to_symbol(&p) {
                sum += p.as_number();
            }
        }
        println!("{sum}");
    }

    fn run_part_two(&self) {
        let r = input_reader::InputReader;
        let data = r.get_as_2d_array("/git/aoc23/src/days/day3/input");
        let schematic = Schematic {
            data
        };

        let mut sum = 0;

        for x in 0..schematic.data.row_len() {
            for y in 0..schematic.data.column_len() {
                if schematic.is_gear(x, y) {
                    let parts = schematic.get_part_numbers_adjacent_to(x,y);
                    sum += parts[0].as_number()*parts[1].as_number();
                }
            }
        }
        println!("{sum}");
    }
}