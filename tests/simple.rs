#[macro_use]
extern crate factori;

#[derive(Debug, Eq, PartialEq)]
struct Vehicle {
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
fn simple_struct_with_accessible_fields() {
    let default = create!(Vehicle);
    assert_eq!(default.number_wheels, 4);
    assert_eq!(default.electric, false);

    let three_wheels = create!(Vehicle, number_wheels: 3);
    assert_eq!(three_wheels.number_wheels, 3);

    let bike = create!(Vehicle, :bike);
    assert_eq!(bike.number_wheels, 2);
    assert_eq!(bike.electric, false);

    let electric_bike = create!(Vehicle, :bike, electric: true);
    assert_eq!(electric_bike.number_wheels, 2);
    assert_eq!(electric_bike.electric, true);

    let another_eletric_bike = create!(Vehicle, :bike, :electric);
    assert_eq!(electric_bike, another_eletric_bike);
}
