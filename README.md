# Factori

A testing factory library for Rust, inspired by [FactoryBot](https://github.com/thoughtbot/factory_bot). 🤖 🦀

Factori works on stable Rust >=1.26. It aims to provide a clean, ergonomic syntax for instantiating test objects, without sacrificing type-safety.

## Example

Factori provides two macros: `factori!`, which defines a factory for a type, and `create!` which instantiates it:

```rust
// Factori's macros recurse a lot. You may need to tell the compiler to increase its macro
// recursion limit. It should not affect run-time performance.
#![recursion_limit="1024"]

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
});

fn main() {
    let default = create!(Vehicle);
    assert_eq!(default.number_wheels, 4);
    assert_eq!(default.electric, false);

    let three_wheels = create!(Vehicle, number_wheels: 3);
    assert_eq!(three_wheels.number_wheels, 3);

    let electric_bike = create!(Vehicle, :bike, electric: true);
    assert_eq!(electric_bike.number_wheels, 2);
    assert_eq!(electric_bike.electric, true);
}
```

More examples are available in the [`tests/`](https://github.com/mjkillough/factori/tree/master/tests) directory.

## Stability

Factori is still brand new, and the syntax may change significantly between point-releases.

## Testing

Run:

```sh
cargo test
```

## License

MIT
