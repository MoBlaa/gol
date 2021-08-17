use naive::{Field, Strategy, ALIVE, DEAD};

// Tests for a field to oscillate after [steps].
fn is_oscillating<const W: usize, const H: usize>(root: Field<W, H>, period: usize) {
    let mut strategy = Strategy::new(root.clone());

    for _ in 1..period {
        let next = strategy.next();
        assert!(next.is_some(), "Not advanced on oscillator",);
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
    )
}
