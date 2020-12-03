extern crate clap;
use clap::{Arg, App, SubCommand};

extern crate advent_of_code_2020;

fn main() {
    /*
    let matches =
        App::new("advent_of_code_2020")
            .author("Christopher Wells <cwellsny@gmail.com>")
            .subcommand(SubCommand::with_name("day1")
                .arg(Arg::with_name("part")
                    .help("Selects the part to run (one, two)")
                    .required(true)
                    .index(1)
                )
            )
        .get_matches()
    ;

    if let Some(matches) = matches.subcommand_matches("day1") {
        let part = matches.value_of("part").unwrap();

        match part {
            "one" => { advent_of_code_2020::one::part_one(); }
            "two" => { advent_of_code_2020::one::part_two(); }
            p => { println!("Unknown part: {}", p); }
        }
    }
    */
}
