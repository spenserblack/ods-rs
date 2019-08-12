# One D Six
![Crates.io](https://img.shields.io/crates/v/one-d-six)
[![docs.rs](https://docs.rs/one-d-six/badge.svg)](https://docs.rs/one-d-six/)
![Crates.io](https://img.shields.io/crates/d/one-d-six)
![Crates.io](https://img.shields.io/crates/l/one-d-six)

Rolls some dice

## Installing
1. Clone this repository
2. Run `make install`

## Usage
### As Binary
```bash
# Print help
one-d-six -h

# Print total of dice rolls
one-d-six 3d4 2d6 1d20

# Print each die cast of each dice roll
one-d-six --complex 2d20 1d12
```

### As Library
*This is not complete usage documentation. This is the expected most common usage.*
```rust

use one_d_six::{
  quickroll,
  Dice,
};

// Quickly generating a set of Dice and rolls them
if quickroll("1d2") == 1 {
    println!("Heads!");
} else {
    println!("Tails!");
}

// Creating sets of dice
let set_1 = Dice::new(2, 4); // Creates 2d4 with Dice::new
let set_2: Dice = "1d20".parse().unwrap() // Creates 1d20 by parsing str

// Combining sets of dice
let mut dice = set_1 + set_2; // Creates 2d4 + 1d20

// Prints 50 rolls of the dice set
for _ in 0..50 {
    dice = dice.roll_all();

    // Method 1
    println!("2d4 + 1d20: {}", dice);

    // Method 2
    let total: u32 = dice.total();
    println!("2d4 + 1d20: {}", total);
}

// Getting value of each die cast
let results = format!("{:?}", dice);
```
