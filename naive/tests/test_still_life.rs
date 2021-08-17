use gol_lib::{Field, ALIVE, DEAD};
use gol_naive::Strategy;

fn is_still(field: Field) {
    let mut strategy = Strategy::new(field);

    let next = strategy.next();
    assert!(
        next.is_none(),
        "Advanced on still life to:\n{}",
        next.unwrap()
    );
}

#[test]
fn test_block() {
    is_still(Field::from([
        [DEAD, DEAD, DEAD, DEAD],
        [DEAD, ALIVE, ALIVE, DEAD],
        [DEAD, ALIVE, ALIVE, DEAD],
        [DEAD, DEAD, DEAD, DEAD],
    ]));
}

#[test]
fn test_beehive() {
    is_still(Field::from([
        [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
        [DEAD, DEAD, ALIVE, ALIVE, DEAD, DEAD],
        [DEAD, ALIVE, DEAD, DEAD, ALIVE, DEAD],
        [DEAD, DEAD, ALIVE, ALIVE, DEAD, DEAD],
        [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
    ]));
}

#[test]
fn test_loaf() {
    is_still(Field::from([
        [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
        [DEAD, DEAD, ALIVE, ALIVE, DEAD, DEAD],
        [DEAD, ALIVE, DEAD, DEAD, ALIVE, DEAD],
        [DEAD, DEAD, ALIVE, DEAD, ALIVE, DEAD],
        [DEAD, DEAD, DEAD, ALIVE, DEAD, DEAD],
        [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
    ]));
}

#[test]
fn test_boat() {
    is_still(Field::from([
        [DEAD, DEAD, DEAD, DEAD, DEAD],
        [DEAD, ALIVE, ALIVE, DEAD, DEAD],
        [DEAD, ALIVE, DEAD, ALIVE, DEAD],
        [DEAD, DEAD, ALIVE, DEAD, DEAD],
        [DEAD, DEAD, DEAD, DEAD, DEAD],
    ]));
}

#[test]
fn test_tub() {
    is_still(Field::from([
        [DEAD, DEAD, DEAD, DEAD, DEAD],
        [DEAD, DEAD, ALIVE, DEAD, DEAD],
        [DEAD, ALIVE, DEAD, ALIVE, DEAD],
        [DEAD, DEAD, ALIVE, DEAD, DEAD],
        [DEAD, DEAD, DEAD, DEAD, DEAD],
    ]));
}
