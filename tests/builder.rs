#![recursion_limit="1024"]

#[macro_use]
extern crate factori;

// Define struct in a separate module. As its fields are private, this means
// we can't use struct literal syntax to construct it.
mod separate_module {
    pub struct Vehicle {
        number_wheels: u8,
        electric: bool,
    }

    impl Vehicle {
        pub fn new(number_wheels: u8, electric: bool) -> Self {
            Vehicle {
                number_wheels,
                electric,
            }
        }

        pub fn number_wheels(&self) -> u8 {
            self.number_wheels
        }

        pub fn electric(&self) -> bool {
            self.electric
        }
    }
}

use separate_module::Vehicle;

factori!(Vehicle, {
    default {
        number_wheels: u8 = 4,
        electric: bool = false,
    }

    builder |fields| {
        Vehicle::new(fields.number_wheels, fields.electric)
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
    assert_eq!(default.number_wheels(), 4);
    assert_eq!(default.electric(), false);
}

#[test]
fn override_field() {
    let three_wheels = create!(Vehicle, number_wheels: 3);
    assert_eq!(three_wheels.number_wheels(), 3);
}

#[test]
fn one_trait() {
    let bike = create!(Vehicle, :bike);
    assert_eq!(bike.number_wheels(), 2);
    assert_eq!(bike.electric(), false);
}

#[test]
fn trait_and_override() {
    let electric_bike = create!(Vehicle, :bike, electric: true);
    assert_eq!(electric_bike.number_wheels(), 2);
    assert_eq!(electric_bike.electric(), true);
}

#[test]
fn two_traits() {
    let electric_bike = create!(Vehicle, :bike, :electric);
    assert_eq!(electric_bike.number_wheels(), 2);
    assert_eq!(electric_bike.electric(), true);
}
