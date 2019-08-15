use clap::{
    Arg,
    App,
};

use one_d_six::Dice;

fn main() {
    let input_arg = Arg::with_name("DICE")
        .help("The dice to be rolled (e.g. 1d6)")
        .required(true)
        .index(1)
        .min_values(1);
    let complexity_arg = Arg::with_name("complexity")
        .short("c")
        .long("complex")
        .help("If you want each cast die to be printed");
    let app_args = App::new("One D Six")
        .version("0.1.0")
        .about("Rolls some dice")
        .arg(input_arg)
        .arg(complexity_arg);
    let matches = app_args.get_matches();
    let rolls = matches.values_of("DICE").unwrap();
    let complex = matches.is_present("complexity");

    let rolls = rolls.map(|r| {
        let dice: Result<Dice<u32>, _> = r.parse();
        (r, dice)
    });

    if complex {
        for (roll, result) in rolls {
            match result {
                Ok(dice) => println!("{}: {:?}", roll, dice),
                Err(e) => eprintln!("{}: {}", roll, e),
            };
        }
    } else {
        for (roll, result) in rolls {
            match result {
                Ok(dice) => println!("{}: {}", roll, dice),
                Err(e) => eprintln!("{}: {}", roll, e),
            };
        }
    }
}
