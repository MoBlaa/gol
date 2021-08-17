use std::fmt;

struct Field<const WIDTH: usize, const HEIGHT: usize> {
    inner: [[char; WIDTH]; HEIGHT]
}

impl<const WIDTH: usize, const HEIGHT: usize> Field<WIDTH, HEIGHT> {
    fn new() -> Self {
        Self { inner: [[' '; WIDTH]; HEIGHT] }
    }

    fn neighbours(&self, (x, y): (usize, usize)) -> Vec<char> {
        if x >= WIDTH || y >= HEIGHT {
            panic!("Out of field bounds: ({}, {})", x, y);
        }

        let range_from = |x: usize, max: usize| match (x > 0, x < max - 1) {
            (true, true) => (x-1)..(x+2),
            (false, true) => x..(x+2),
            (true, false) => (x-1)..(x+1),
            (false, false) => x..(x+1),
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

    fn value(&self, (x, y): (usize, usize)) -> &char {
        &self.inner[y][x]
    }

    fn value_mut(&mut self, (x, y): (usize, usize)) -> &mut char {
        &mut self.inner[y][x]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Display for Field<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.inner {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut field = Field::<5, 5>::new();
    *field.value_mut((1, 1)) = '0';
    *field.value_mut((2, 1)) = '1';
    *field.value_mut((3, 1)) = '2';
    *field.value_mut((1, 2)) = '3';
    *field.value_mut((2, 2)) = 'x';
    *field.value_mut((3, 2)) = '4';
    *field.value_mut((1, 3)) = '5';
    *field.value_mut((2, 3)) = '6';
    *field.value_mut((3, 3)) = '7';
    println!("Hello, Field:\n{}", field);
    let neighbours = field.neighbours((2, 2));
    println!("Neighbours of (x=2, y=2) are: {:?}", neighbours);
}

