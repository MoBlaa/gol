use gol_lib::{Field, ALIVE, DEAD};

pub struct Update((usize, usize), char);

impl Update {
    pub fn into_inner(self) -> ((usize, usize), char) {
        (self.0, self.1)
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
    pub fn advance_one(cords: (usize, usize), field: &Field) -> Option<char> {
        let neighbours = field.neighbours(cords);
        let value = field.value(cords);

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

    pub fn advance_row(row: usize, field: &Field) -> Vec<Update> {
        let mut updates = Vec::new();
        for column in 0..field.width() {
            if let Some(update) = Strategy::advance_one((column, row), field) {
                updates.push(Update((column, row), update));
            }
        }
        updates
    }
}

impl Iterator for Strategy {
    type Item = Field;

    fn next(&mut self) -> Option<Self::Item> {
        let mut field = self.field.clone();

        let mut updated_any = false;
        for row in 0..self.field.height() {
            let updates = Strategy::advance_row(row, &self.field);
            for Update(cords, value) in updates {
                *field.value_mut(cords) = value;
                updated_any = true;
            }
        }

        if !updated_any {
            return None;
        }

        self.field = field.clone();

        Some(field)
    }
}
