use futures::{Stream, StreamExt};
use gol_lib::{Field, ALIVE, DEAD};
use itertools::Itertools;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Strategy {
    field: Field,
}

impl Strategy {
    pub fn new(field: Field) -> Self {
        Strategy { field }
    }

    /// Returns the resulting value of one cell if it changes.
    pub async fn advance_one(cords: (usize, usize), field: Arc<RwLock<Field>>) -> Option<char> {
        let (neighbours, value) = {
            let field = field.read().await;
            (field.neighbours(cords), *field.value(cords))
        };

        let alive = neighbours.iter().filter(|char| char == &&ALIVE).count();

        // Breakdown of the rules
        /*
        1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        2. Any live cell with two or three live neighbours lives on to the next generation.
        3. Any live cell with more than three live neighbours dies, as if by overpopulation.
        4.Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
         */
        match (value, alive < 2, alive == 2, alive == 3, alive > 3) {
            (ALIVE, true, _, _, _) => Some(DEAD), // underpopulation
            (ALIVE, _, true, _, _) => None,       // next generation
            (ALIVE, _, _, true, _) => None,       // next generation
            (ALIVE, _, _, _, true) => Some(DEAD), // overpopulation
            (DEAD, _, _, true, _) => Some(ALIVE), // reproduction
            _ => None,
        }
    }

    async fn advance_row(
        row: usize,
        field: Arc<RwLock<Field>>,
    ) -> impl Stream<Item = ((usize, usize), char)> {
        let width = field.read().await.width();
        async_stream::stream! {
            for x in 0..width {
                if let Some(value) = Strategy::advance_one((x, row), Arc::clone(&field)).await {
                    let item = ((x, row), value);
                    yield item;
                }
            }
        }
    }

    async fn advance_field(field: Arc<RwLock<Field>>) -> Option<()> {
        let height = field.read().await.height();
        let mut receiver = {
            let (sender, receiver) = tokio::sync::mpsc::channel(1000);
            for chunk in &(0..height).chunks(2 * height / num_cpus::get()) {
                let field = Arc::clone(&field);
                let chunk = chunk.collect::<Vec<_>>();
                let sender = sender.clone();
                tokio::spawn(async move {
                    for y in chunk {
                        let field = Arc::clone(&field);
                        let stream = Strategy::advance_row(y, field).await;
                        futures::pin_mut!(stream);
                        while let Some(item) = stream.next().await {
                            sender.send(item).await.expect("Failed to send update");
                        }
                    }
                });
            }
            receiver
        };

        let consumer = tokio::spawn(async move {
            let mut updates_some = false;
            while let Some((cords, char)) = receiver.recv().await {
                let mut field = field.write().await;
                *field.value_mut(cords) = char;
                updates_some = true;
            }
            updates_some
        });

        if !consumer.await.unwrap_or(false) {
            return None;
        }

        Some(())
    }

    pub fn into_stream(self) -> impl Stream<Item = Field> {
        let field = Arc::new(RwLock::new(self.field));
        async_stream::stream! {
            loop {
                let response = Strategy::advance_field(Arc::clone(&field)).await;
                if response.is_some() {
                    let field = field.read().await.clone();
                    yield field;
                } else {
                    break;
                }
            }
        }
    }
}
