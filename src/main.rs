extern crate time;

use std::collections::HashMap;
use std::env;

mod char_grid;

use char_grid::CharGrid;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Parameter {
    RepositoryName,
    Text,
    None,
}

fn match_parameter(parameter: &str) -> Parameter {
    match parameter.as_ref() {
        "repo" => Parameter::RepositoryName,
        "text" => Parameter::Text,
        _ => panic!("Unknown parameter: <{}>", parameter),
    }
}

fn read_cli_arguments() -> HashMap<Parameter, String> {
    let mut previous_arg: Parameter = Parameter::None;
    let mut parameters = HashMap::new();
    for arg in env::args().skip(1) {
        if arg.starts_with("--") {
            previous_arg = match_parameter(&arg[2..]);
        } else if previous_arg == Parameter::None {
            panic!("No flag corresponding to parameter <{}>", arg);
        } else {
            parameters.insert(previous_arg, arg);
            previous_arg = Parameter::None;
        }
    }
    parameters
}

fn get_next_sunday(t: time::Tm) -> time::Tm {
    let days: i32 = 7 - t.tm_wday;
    t + time::Duration::days(days as i64)
}

fn get_sunday_of_last_year(t: time::Tm) -> time::Tm {
    get_next_sunday(get_next_sunday(t) - time::Duration::days(365))
}

fn main() {
    let params = read_cli_arguments();
    println!("{}", time::now().rfc822());
    println!("{}", get_sunday_of_last_year(time::now()).rfc822());
    let cg = CharGrid::new();
    let s = "paul";
    for c in s.chars() {
        for w in cg.get_grid_for(c) {
            for i in w {
                if i > &0 {
                    print!("x");
                } else {
                    print!(".");
                }
            }
            println!("\n");
        }
    }
}

