use std::fmt;
use std::fmt::Write;

pub const ALIVE: char = '\u{25AE}';
pub const DEAD: char = '\u{25AF}';

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Field {
    inner: Vec<Vec<char>>,
}

impl Field {
    pub fn random(width: usize, height: usize) -> Self {
        let mut field = Self::dead(width, height);

        for x in 0..width {
            for y in 0..height {
                *field.value_mut((x, y)) = if rand::random::<bool>() { ALIVE } else { DEAD }
            }
        }

        field
    }

    pub fn dead(width: usize, height: usize) -> Self {
        if width < 3 || height < 3 {
            panic!("minimum size of a field is 3x3");
        }
        let mut inner = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(DEAD);
            }
            inner.push(row);
        }
        Self { inner }
    }

    pub fn height(&self) -> usize {
        self.inner.len()
    }

    pub fn width(&self) -> usize {
        self.inner[0].len()
    }

    pub fn neighbours(&self, (x, y): (usize, usize)) -> Vec<char> {
        if y >= self.height() || x >= self.width() {
            panic!("Out of field bounds: ({}, {})", x, y);
        }

        let range_from = |x: usize, max: usize| match (x > 0, x < max - 1) {
            (true, true) => (x - 1)..(x + 2),
            (false, true) => x..(x + 2),
            (true, false) => (x - 1)..(x + 1),
            (false, false) => x..(x + 1),
        };

        let mut result = Vec::with_capacity(8);
        let x_range = range_from(x, self.width());
        let y_range = range_from(y, self.height());

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

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut frame = String::new();
        write!(&mut frame, "[")?;
        for _ in 0..(self.width() - 2) {
            write!(&mut frame, "-")?;
        }
        write!(&mut frame, "]")?;
        writeln!(f, "{}", frame)?;
        for row in &self.inner {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        write!(f, "{}", frame)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> From<[[char; WIDTH]; HEIGHT]> for Field {
    fn from(array: [[char; WIDTH]; HEIGHT]) -> Self {
        let mut inner = Vec::with_capacity(HEIGHT);
        for row in array {
            inner.push(Vec::from(row));
        }
        Self { inner }
    }
}

pub struct Strategy {
    field: Field,
}

impl Strategy {
    pub fn new(field: Field) -> Self {
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
        match (value, alive < 2, alive == 2, alive == 3, alive > 3) {
            (&ALIVE, true, _, _, _) => Some(DEAD), // underpopulation
            (&ALIVE, _, true, _, _) => None,       // next generation
            (&ALIVE, _, _, true, _) => None,       // next generation
            (&ALIVE, _, _, _, true) => Some(DEAD), // overpopulation
            (&DEAD, _, _, true, _) => Some(ALIVE), // reproduction
            _ => None,
        }
    }
}

impl Iterator for Strategy {
    type Item = Field;

    fn next(&mut self) -> Option<Self::Item> {
        let mut field = self.field.clone();

        let mut updated_any = false;
        for x in 0..self.field.width() {
            for y in 0..self.field.height() {
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
