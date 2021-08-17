use naive::{Field, Strategy};
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let field = Field::<100, 10>::random();
    println!("Round 0:\n{}", field);

    let mut strategy = Strategy::new(field);

    let mut round = 1usize;
    loop {
        tokio::time::sleep(Duration::from_millis(500)).await;

        let field = strategy.next();
        if field.is_none() {
            println!("Extinction is inevitable...");
            break;
        }
        let field = field.unwrap();

        println!("Round {}:\n{}", round, field);
        round = round + 1;
    }
}
