use gol_lib::{Field, ALIVE, DEAD};

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
