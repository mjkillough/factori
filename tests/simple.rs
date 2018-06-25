#![recursion_limit="512"]

#[macro_use]
extern crate factori;

pub struct Vehicle {
    number_wheels: u8,
    electric: bool,
}

factori!(Vehicle, {
    default {
        number_wheels: 4,
        electric: false,
    }

    trait bike {
        number_wheels: 2,
    }

    trait electric {
        electric: true,
    }
});

#[test]
fn simple_struct() {
    let default = create!(Vehicle);
    assert_eq!(default.number_wheels, 4);
    assert_eq!(default.electric, false);
}

#[test]
fn override_field() {
    let three_wheels = create!(Vehicle, number_wheels: 3);
    assert_eq!(three_wheels.number_wheels, 3);
}

#[test]
fn one_trait() {
    let bike = create!(Vehicle, :bike);
    assert_eq!(bike.number_wheels, 2);
    assert_eq!(bike.electric, false);
}

#[test]
fn trait_and_override() {
    let electric_bike = create!(Vehicle, :bike, electric: true);
    assert_eq!(electric_bike.number_wheels, 2);
    assert_eq!(electric_bike.electric, true);
}

#[test]
fn two_traits() {
    let electric_bike = create!(Vehicle, :bike, :electric);
    assert_eq!(electric_bike.number_wheels, 2);
    assert_eq!(electric_bike.electric, true);
}
