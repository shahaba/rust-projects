#[cfg(test)]
use automated_tests::rectangle::Rectangle;

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        width: 20,
        length: 25,
    };

    let smaller = Rectangle {
        width: 10,
        length: 15,
    };

    assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_cant_hold_larger() {
    let larger = Rectangle {
        width: 20,
        length: 25,
    };

    let smaller = Rectangle {
        width: 10,
        length: 15,
    };

    assert!(!smaller.can_hold(&larger));
}
