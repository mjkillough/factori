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

    builder {
        Vehicle::new(number_wheels, electric)
    }

    mixin bike {
        number_wheels = 2,
    }

    mixin electric {
        electric = true,
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
fn one_mixin() {
    let bike = create!(Vehicle, :bike);
    assert_eq!(bike.number_wheels(), 2);
    assert_eq!(bike.electric(), false);
}

#[test]
fn mixin_and_override() {
    let electric_bike = create!(Vehicle, :bike, electric: true);
    assert_eq!(electric_bike.number_wheels(), 2);
    assert_eq!(electric_bike.electric(), true);
}

#[test]
fn two_mixins() {
    let electric_bike = create!(Vehicle, :bike, :electric);
    assert_eq!(electric_bike.number_wheels(), 2);
    assert_eq!(electric_bike.electric(), true);
}
