use futures::Stream;
use gol_lib::{Field, ALIVE, DEAD};
use itertools::Itertools;

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

    fn advance_row(row: usize, field: &Field) -> Vec<((usize, usize), char)> {
        let mut updates = Vec::new();
        for x in 0..field.width() {
            if let Some(value) = Strategy::advance_one((x, row), &field) {
                updates.push(((x, row), value));
            }
        }
        updates
    }

    async fn advance_field(mut field: Field) -> Option<Field> {
        let mut updates = Vec::with_capacity(field.width());
        for chunk in &(0..field.height()).chunks(field.height() / 4 * num_cpus::get()) {
            let field = field.clone();
            let chunk = chunk.collect::<Vec<_>>();
            updates.push(tokio::spawn(async move {
                let mut updates = Vec::new();
                for y in chunk {
                    updates.extend(Strategy::advance_row(y, &field));
                }
                updates
            }));
        }

        let updates = futures::future::join_all(updates)
            .await
            .into_iter()
            .flatten()
            .flat_map(|e| e.into_iter())
            .collect::<Vec<((usize, usize), char)>>();

        if updates.is_empty() {
            return None;
        }

        // Update field
        for (cords, char) in updates {
            *field.value_mut(cords) = char;
        }
        Some(field)
    }

    pub fn into_stream(self) -> impl Stream<Item = Field> {
        let mut old_field = self.field;
        async_stream::stream! {
            loop {
                let field = Strategy::advance_field(old_field.clone()).await;
                if let Some(field) = field {
                    yield field.clone();
                    old_field = field;
                } else {
                    break;
                }
            }
        }
    }
}
