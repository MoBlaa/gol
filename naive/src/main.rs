use gol_naive::{Field, Strategy};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use tokio::time::{Duration, Instant};

fn hash(field: &Field) -> u64 {
    let mut hasher = DefaultHasher::new();
    field.hash(&mut hasher);
    hasher.finish()
}

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    args.next();
    let width: usize = args.next().map(|s| s.parse().unwrap_or(100)).unwrap_or(100);

    let height: usize = args.next().map(|s| s.parse().unwrap_or(100)).unwrap_or(100);
    let timeout: u64 = args.next().map(|s| s.parse().unwrap_or(250)).unwrap_or(250);

    let field = Field::random(width, height);
    println!("Round 0:\n{}", field);

    let mut strategy = Strategy::new(field.clone());

    let mut round = 1usize;
    let mut visited = HashSet::new();
    visited.insert(hash(&field));
    loop {
        tokio::time::sleep(Duration::from_millis(timeout)).await;

        let now = Instant::now();
        let field = strategy.next();
        let elapsed = now.elapsed();
        if field.is_none() {
            println!("Reached stable state after {} rounds.", round + 1);
            break;
        }
        let field = field.unwrap();
        if !visited.insert(hash(&field)) {
            println!("Oscillation reached after {} rounds.", round + 1);
            break;
        }

        println!("Round {} ({:?}):\n{}", round, elapsed, field);
        round += 1;
    }
}
