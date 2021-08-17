use std::fmt;
use std::fmt::Write;

pub const ALIVE: char = '▣';
pub const DEAD: char = '▢';

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Field<const WIDTH: usize, const HEIGHT: usize> {
    inner: [[char; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Field<WIDTH, HEIGHT> {
    pub fn random() -> Self {
        let mut field = Self::dead();

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                *field.value_mut((x, y)) = if rand::random::<bool>() { ALIVE } else { DEAD }
            }
        }

        field
    }

    pub fn dead() -> Self {
        if WIDTH < 3 || HEIGHT < 3 {
            panic!("minimum size of a field is 3x3");
        }
        Self {
            inner: [[DEAD; WIDTH]; HEIGHT],
        }
    }

    pub fn neighbours(&self, (x, y): (usize, usize)) -> Vec<char> {
        if x >= WIDTH || y >= HEIGHT {
            panic!("Out of field bounds: ({}, {})", x, y);
        }

        let range_from = |x: usize, max: usize| match (x > 0, x < max - 1) {
            (true, true) => (x - 1)..(x + 2),
            (false, true) => x..(x + 2),
            (true, false) => (x - 1)..(x + 1),
            (false, false) => x..(x + 1),
        };

        let mut result = Vec::with_capacity(8);
        let x_range = range_from(x, WIDTH);
        let y_range = range_from(y, HEIGHT);

        for column in x_range {
            for row in y_range.clone() {
                if column != x || row != y {
                    result.push(self.inner[row][column]);
                }
            }
        }
        result
    }

    pub fn value(&self, (x, y): (usize, usize)) -> &char {
        &self.inner[y][x]
    }

    pub fn value_mut(&mut self, (x, y): (usize, usize)) -> &mut char {
        &mut self.inner[y][x]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Display for Field<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut frame = String::new();
        write!(&mut frame, "[")?;
        for _ in 0..(WIDTH - 2) {
            write!(&mut frame, "-")?;
        }
        write!(&mut frame, "]")?;
        writeln!(f, "{}", frame)?;
        for row in self.inner {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        write!(f, "{}", frame)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> From<[[char; WIDTH]; HEIGHT]>
    for Field<WIDTH, HEIGHT>
{
    fn from(inner: [[char; WIDTH]; HEIGHT]) -> Self {
        Self { inner }
    }
}

pub struct Strategy<const WIDTH: usize, const HEIGHT: usize> {
    field: Field<WIDTH, HEIGHT>,
}

impl<const WIDTH: usize, const HEIGHT: usize> Strategy<WIDTH, HEIGHT> {
    pub fn new(field: Field<WIDTH, HEIGHT>) -> Self {
        Strategy { field }
    }

    /// Returns the resulting value of one cell if it changes.
    pub fn advance_one(&self, cords: (usize, usize)) -> Option<char> {
        let neighbours = self.field.neighbours(cords);
        let value = self.field.value(cords);

        let alive = neighbours.iter().filter(|char| char == &&ALIVE).count();

        // Breakdown of the rules
        /*
        1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        2. Any live cell with two or three live neighbours lives on to the next generation.
        3. Any live cell with more than three live neighbours dies, as if by overpopulation.
        4.Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
         */
        match (value, alive == 2, alive == 3) {
            (&ALIVE, true, _) => None,
            (&ALIVE, _, true) => None,
            (&DEAD, _, true) => Some(ALIVE),
            (&ALIVE, _, _) => Some(DEAD),
            _ => None,
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Iterator for Strategy<WIDTH, HEIGHT> {
    type Item = Field<WIDTH, HEIGHT>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut field = Field::dead();

        let mut updated_any = false;
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if let Some(value) = self.advance_one((x, y)) {
                    *field.value_mut((x, y)) = value;
                    updated_any = true;
                }
            }
        }

        if !updated_any {
            return None;
        }

        self.field = field.clone();

        Some(field)
    }
}
