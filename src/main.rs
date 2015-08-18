use std::collections::HashMap;
use std::env;


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

fn print_parameters(parameters: &HashMap<Parameter, String>) {
    for (key, value) in parameters {
        println!("[{:?}] -> {}", key, value);
    }
}

fn main() {
    let params = read_cli_arguments();
    print_parameters(&params);
}

