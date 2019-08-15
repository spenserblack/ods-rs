use one_d_six::{
  quickroll,
  Dice,
};

#[test]
fn usage_example() {
    // Quickly generating a set of Dice and rolls them
    let coinflip: u8 = quickroll("1d2");
    if coinflip == 1 {
        println!("Heads!");
    } else {
        println!("Tails!");
    }

    // Creating sets of dice
    let set_1 = Dice::new(2, 4); // Creates 2d4 with Dice::new
    let set_2: Dice = "1d20".parse().unwrap(); // Creates 1d20 by parsing str

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
    let _results = format!("{:?}", dice);
}
