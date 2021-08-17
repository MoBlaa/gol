use naive::{Field, Strategy, ALIVE, DEAD};

#[test]
fn test_block() {
    let field = Field::from([
        [DEAD, DEAD, DEAD, DEAD],
        [DEAD, ALIVE, ALIVE, DEAD],
        [DEAD, ALIVE, ALIVE, DEAD],
        [DEAD, DEAD, DEAD, DEAD],
    ]);

    let mut strategy = Strategy::new(field.clone());

    let next = strategy.next();
    assert!(
        next.is_none(),
        "Advanced on still life to:\n{}",
        next.unwrap()
    );
}

#[test]
fn test_beehive() {
    let field = Field::from([
        [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
        [DEAD, DEAD, ALIVE, ALIVE, DEAD, DEAD],
        [DEAD, ALIVE, DEAD, DEAD, ALIVE, DEAD],
        [DEAD, DEAD, ALIVE, ALIVE, DEAD, DEAD],
        [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
    ]);

    let mut strategy = Strategy::new(field.clone());

    let next = strategy.next();
    assert!(
        next.is_none(),
        "Advanced on still life to:\n{}",
        next.unwrap()
    );
}

#[test]
fn test_loaf() {
    let field = Field::from([
        [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
        [DEAD, DEAD, ALIVE, ALIVE, DEAD, DEAD],
        [DEAD, ALIVE, DEAD, DEAD, ALIVE, DEAD],
        [DEAD, DEAD, ALIVE, DEAD, ALIVE, DEAD],
        [DEAD, DEAD, DEAD, ALIVE, DEAD, DEAD],
        [DEAD, DEAD, DEAD, DEAD, DEAD, DEAD],
    ]);

    let mut strategy = Strategy::new(field.clone());

    let next = strategy.next();
    assert!(
        next.is_none(),
        "Advanced on still life to:\n{}",
        next.unwrap()
    );
}

#[test]
fn test_boat() {
    let field = Field::from([
        [DEAD, DEAD, DEAD, DEAD, DEAD],
        [DEAD, ALIVE, ALIVE, DEAD, DEAD],
        [DEAD, ALIVE, DEAD, ALIVE, DEAD],
        [DEAD, DEAD, ALIVE, DEAD, DEAD],
        [DEAD, DEAD, DEAD, DEAD, DEAD],
    ]);

    let mut strategy = Strategy::new(field.clone());

    let next = strategy.next();
    assert!(
        next.is_none(),
        "Advanced on still life to:\n{}",
        next.unwrap()
    );
}

#[test]
fn test_tub() {
    let field = Field::from([
        [DEAD, DEAD, DEAD, DEAD, DEAD],
        [DEAD, DEAD, ALIVE, DEAD, DEAD],
        [DEAD, ALIVE, DEAD, ALIVE, DEAD],
        [DEAD, DEAD, ALIVE, DEAD, DEAD],
        [DEAD, DEAD, DEAD, DEAD, DEAD],
    ]);

    let mut strategy = Strategy::new(field.clone());

    let next = strategy.next();
    assert!(
        next.is_none(),
        "Advanced on still life to:\n{}",
        next.unwrap()
    );
}
