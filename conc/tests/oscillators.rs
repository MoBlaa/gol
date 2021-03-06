use gol_conc::Strategy;
use gol_lib::{Field, ALIVE, DEAD};

// Tests for a field to oscillate after [steps].
fn is_oscillating(root: Field, period: usize) {
    let mut strategy = Strategy::new(root.clone());

    for period in 1..period {
        let next = strategy.next();
        assert!(next.is_some(), "Not advanced on oscillator",);
        println!("Period {}:\n{}", period, next.unwrap());
    }

    let repetition = strategy.next();
    assert!(repetition.is_some(), "Not advanced on oscillator",);
    let repetition = repetition.unwrap();
    assert_eq!(root, repetition);
}

#[test]
fn test_blinker() {
    is_oscillating(
        Field::from([
            [DEAD, DEAD, DEAD, DEAD, DEAD],
            [DEAD, DEAD, ALIVE, DEAD, DEAD],
            [DEAD, DEAD, ALIVE, DEAD, DEAD],
            [DEAD, DEAD, ALIVE, DEAD, DEAD],
            [DEAD, DEAD, DEAD, DEAD, DEAD],
        ]),
        2,
    );

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
}

#[test]
fn test_toad() {
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
    );
}

#[test]
fn test_beacon() {
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
    );
}

#[test]
fn test_pulsar() {
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
    );
}

#[test]
fn test_pentadecathlon() {
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
    );
}
