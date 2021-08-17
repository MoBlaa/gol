use futures::StreamExt;
use gol_adv::Strategy;
use gol_lib::Field;
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
    let print: bool = args
        .next()
        .map(|s| s.parse().unwrap_or(false))
        .unwrap_or(false);

    let field = Field::random(width, height);
    if print {
        println!("Round 0:\n{}", field);
    }

    let strategy = Strategy::new(field.clone());

    let mut round = 1u32;
    let mut visited = HashSet::new();
    visited.insert(hash(&field));
    let mut whole = Duration::new(0, 0);

    let field_stream = strategy.into_stream();

    futures::pin_mut!(field_stream);

    loop {
        let now = Instant::now();
        let field = match field_stream.next().await {
            None => break,
            Some(field) => field,
        };
        let elapsed = now.elapsed();

        whole += elapsed;

        if !visited.insert(hash(&field)) {
            break;
        }

        if print {
            println!("Round {} ({:?})", round, elapsed);
            println!("{}", field);
        }
        round += 1;

        tokio::time::sleep(Duration::from_millis(timeout)).await;
    }
    println!(
        "Finished after {}rnd and {:?} ({:?}/s)",
        round + 1,
        whole,
        whole / (round + 1)
    );
}
