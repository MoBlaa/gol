use futures::StreamExt;
use gol_adv::Strategy;
use gol_lib::{Field, ALIVE, DEAD};

// Tests for a field to oscillate after [steps].
async fn is_oscillating(root: Field, period: usize) {
    let strategy = Strategy::new(root.clone()).into_stream();

    futures::pin_mut!(strategy);

    for period in 1..period {
        let next = strategy.next().await;
        assert!(next.is_some(), "Not advanced on oscillator",);
        println!("Period {}:\n{}", period, next.unwrap());
    }

    let repetition = strategy.next().await;
    assert!(repetition.is_some(), "Not advanced on oscillator",);
    let repetition = repetition.unwrap();
    assert_eq!(root, repetition);
}

#[tokio::test]
async fn test_blinker() {
    is_oscillating(
        Field::from([
            [DEAD, DEAD, DEAD, DEAD, DEAD],
            [DEAD, DEAD, ALIVE, DEAD, DEAD],
            [DEAD, DEAD, ALIVE, DEAD, DEAD],
            [DEAD, DEAD, ALIVE, DEAD, DEAD],
            [DEAD, DEAD, DEAD, DEAD, DEAD],
        ]),
        2,
    )
    .await;

    is_oscillating(
        Field::from([
            [DEAD, DEAD, DEAD, DEAD, DEAD],
            [DEAD, DEAD, DEAD, DEAD, DEAD],
            [DEAD, ALIVE, ALIVE, ALIVE, DEAD],
            [DEAD, DEAD, DEAD, DEAD, DEAD],
            [DEAD, DEAD, DEAD, DEAD, DEAD],
        ]),
        2,
    )
    .await;
}

#[tokio::test]
async fn test_toad() {
    is_oscillating(
        Field::from([
            [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
            [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
            [DEAD, DEAD, ALIVE, ALIVE, ALIVE, DEAD],
            [DEAD, ALIVE, ALIVE, ALIVE, DEAD, DEAD],
            [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
            [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
        ]),
        2,
    )
    .await;
}

#[tokio::test]
async fn test_beacon() {
    is_oscillating(
        Field::from([
            [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
            [DEAD, ALIVE, ALIVE, DEAD, DEAD, DEAD],
            [DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD],
            [DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD],
            [DEAD, DEAD, DEAD, ALIVE, ALIVE, DEAD],
            [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
        ]),
        2,
    )
    .await;
}

#[tokio::test]
async fn test_pulsar() {
    is_oscillating(
        Field::from([
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD,
                DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD,
                DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, ALIVE, ALIVE, ALIVE, DEAD, DEAD, DEAD, ALIVE, ALIVE, ALIVE,
                DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD,
                DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD, ALIVE, DEAD, DEAD, DEAD,
                DEAD, ALIVE, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD, ALIVE, DEAD, DEAD, DEAD,
                DEAD, ALIVE, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD, ALIVE, DEAD, DEAD, DEAD,
                DEAD, ALIVE, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, ALIVE, ALIVE, ALIVE, DEAD, DEAD, DEAD, ALIVE, ALIVE, ALIVE,
                DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD,
                DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, ALIVE, ALIVE, ALIVE, DEAD, DEAD, DEAD, ALIVE, ALIVE, ALIVE,
                DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD, ALIVE, DEAD, DEAD, DEAD,
                DEAD, ALIVE, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD, ALIVE, DEAD, DEAD, DEAD,
                DEAD, ALIVE, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD, ALIVE, DEAD, DEAD, DEAD,
                DEAD, ALIVE, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD,
                DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, ALIVE, ALIVE, ALIVE, DEAD, DEAD, DEAD, ALIVE, ALIVE, ALIVE,
                DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD,
                DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD,
                DEAD, DEAD, DEAD,
            ],
        ]),
        3,
    )
    .await;
}

#[tokio::test]
async fn test_pentadecathlon() {
    is_oscillating(
        Field::from([
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, ALIVE, DEAD, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD,
            ],
            [
                DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD, DEAD,
            ],
        ]),
        15,
    )
    .await;
}
