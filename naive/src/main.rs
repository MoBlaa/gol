use std::fmt;
use std::fmt::Write;

struct Field<const WIDTH: usize, const HEIGHT: usize> {
    inner: [[char; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Field<WIDTH, HEIGHT> {
    fn random() -> Self {
        let mut field = Self::new();

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                *field.value_mut((x, y)) = if rand::random::<bool>() { '▣' } else { '▢' }
            }
        }

        field
    }

    fn new() -> Self {
        if WIDTH < 3 || HEIGHT < 3 {
            panic!("minimum size of a field is 3x3");
        }
        Self {
            inner: [[' '; WIDTH]; HEIGHT],
        }
    }

    fn neighbours(&self, (x, y): (usize, usize)) -> Vec<char> {
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

    fn value(&self, (x, y): (usize, usize)) -> &char {
        &self.inner[y][x]
    }

    fn value_mut(&mut self, (x, y): (usize, usize)) -> &mut char {
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

#[tokio::main]
async fn main() {
    let field = Field::<20, 10>::random();
    println!("Hello, Field:\n{}", field);

    let print_neighbours = |cords: (usize, usize)| {
        let neighbours = field.neighbours(cords);
        println!(
            "Neighbours of '{}' {:?} are: {:?}",
            field.value(cords),
            cords,
            neighbours
        );
    };

    print_neighbours((0, 0));
    print_neighbours((19, 0));
    print_neighbours((0, 9));
    print_neighbours((19, 9));
}
