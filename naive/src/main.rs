use naive::{Field, Strategy};
use tokio::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let field = Field::<100, 10>::random();
    println!("Round 0:\n{}", field);

    let mut strategy = Strategy::new(field);

    let mut round = 1usize;
    loop {
        tokio::time::sleep(Duration::from_millis(250)).await;

        let now = Instant::now();
        let field = strategy.next();
        let elapsed = now.elapsed();
        if field.is_none() {
            println!("Reached stable state after {} rounds.", round + 1);
            break;
        }
        let field = field.unwrap();

        println!("Round {} ({:?}):\n{}", round, elapsed, field);
        round = round + 1;
    }
}
