use futures::StreamExt;
use gol_adv::Strategy;
use gol_lib::{Field, ALIVE, DEAD};

async fn is_still(field: Field) {
    let strategy = Strategy::new(field).into_stream();

    futures::pin_mut!(strategy);

    let next = strategy.next().await;
    assert!(
        next.is_none(),
        "Advanced on still life to:\n{}",
        next.unwrap()
    );
}

#[tokio::test]
async fn test_block() {
    is_still(Field::from([
        [DEAD, DEAD, DEAD, DEAD],
        [DEAD, ALIVE, ALIVE, DEAD],
        [DEAD, ALIVE, ALIVE, DEAD],
        [DEAD, DEAD, DEAD, DEAD],
    ]))
    .await;
}

#[tokio::test]
async fn test_beehive() {
    is_still(Field::from([
        [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
        [DEAD, DEAD, ALIVE, ALIVE, DEAD, DEAD],
        [DEAD, ALIVE, DEAD, DEAD, ALIVE, DEAD],
        [DEAD, DEAD, ALIVE, ALIVE, DEAD, DEAD],
        [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
    ]))
    .await;
}

#[tokio::test]
async fn test_loaf() {
    is_still(Field::from([
        [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
        [DEAD, DEAD, ALIVE, ALIVE, DEAD, DEAD],
        [DEAD, ALIVE, DEAD, DEAD, ALIVE, DEAD],
        [DEAD, DEAD, ALIVE, DEAD, ALIVE, DEAD],
        [DEAD, DEAD, DEAD, ALIVE, DEAD, DEAD],
        [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
    ]))
    .await;
}

#[tokio::test]
async fn test_boat() {
    is_still(Field::from([
        [DEAD, DEAD, DEAD, DEAD, DEAD],
        [DEAD, ALIVE, ALIVE, DEAD, DEAD],
        [DEAD, ALIVE, DEAD, ALIVE, DEAD],
        [DEAD, DEAD, ALIVE, DEAD, DEAD],
        [DEAD, DEAD, DEAD, DEAD, DEAD],
    ]))
    .await;
}

#[tokio::test]
async fn test_tub() {
    is_still(Field::from([
        [DEAD, DEAD, DEAD, DEAD, DEAD],
        [DEAD, DEAD, ALIVE, DEAD, DEAD],
        [DEAD, ALIVE, DEAD, ALIVE, DEAD],
        [DEAD, DEAD, ALIVE, DEAD, DEAD],
        [DEAD, DEAD, DEAD, DEAD, DEAD],
    ]))
    .await;
}
