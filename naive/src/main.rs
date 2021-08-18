use gol_lib::Field;
use gol_naive::Strategy;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use tokio::time::{Duration, Instant};

fn hash(field: &Field) -> u64 {
    let mut hasher = DefaultHasher::new();
    field.hash(&mut hasher);
    hasher.finish()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args();
    args.next();
    let width: u32 = args.next().map(|s| s.parse().unwrap_or(100)).unwrap_or(100);

    let height: u32 = args.next().map(|s| s.parse().unwrap_or(100)).unwrap_or(100);
    let timeout: u64 = args.next().map(|s| s.parse().unwrap_or(250)).unwrap_or(250);
    let print: bool = args
        .next()
        .map(|s| s.parse().unwrap_or(false))
        .unwrap_or(false);

    let field = Field::random(
        usize::try_from(width).unwrap(),
        usize::try_from(height).unwrap(),
    );
    if print {
        println!("Round 0:\n{}", field);
    }

    let mut strategy = Strategy::new(field.clone());

    let mut round = 1u32;
    let mut visited = HashSet::new();
    visited.insert(hash(&field));
    let mut whole = Duration::new(0, 0);
    loop {
        tokio::time::sleep(Duration::from_millis(timeout)).await;

        let now = Instant::now();
        let field = strategy.next();
        let elapsed = now.elapsed();

        whole += elapsed;

        if field.is_none() {
            break;
        }
        let field = field.unwrap();
        if !visited.insert(hash(&field)) {
            break;
        }

        if print {
            println!("Round {} ({:?})", round, elapsed);
            println!("{}", field);
        }
        round += 1;
    }
    println!(
        "Finished after {}rnd and {:?} ({:?} pro Runde und Feld)",
        round + 1,
        whole,
        whole / (round + 1 + (height * width))
    );
}
