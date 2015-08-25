extern crate git2;
extern crate time;

use std::collections::HashMap;
use std::env;

mod char_grid;

use char_grid::CharGrid;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Parameter {
    RepositoryName,
    RepositoryPath,
    Text,
    LowCommitCount,
    HighCommitCount,
    None,
}

fn match_parameter(parameter: &str) -> Parameter {
    match parameter.as_ref() {
        "repo" => Parameter::RepositoryName,
        "path" => Parameter::RepositoryPath,
        "text" => Parameter::Text,
        "low" => Parameter::LowCommitCount,
        "high" => Parameter::HighCommitCount,
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

fn get_sunday_n_weeks_before(t: time::Tm, n: i32) -> time::Tm {
    get_next_sunday(get_next_sunday(t) - time::Duration::days((n*7) as i64))
}

fn write_to_repository(repo: &mut git2::Repository, low: i32, high: i32, s: String) {
    let mut previous_commit: Option<git2::Commit> = None;
    let mut commit_time = get_sunday_n_weeks_before(time::now(), (s.len()*4) as i32);
    let oid = repo.index().unwrap().write_tree().unwrap();
    let cg = CharGrid::new();
    for c in s.chars() {
        let array = cg.get_grid_for(c);
        let height = array.len();
        let width = array[0].len();
        for j in 0..width {
            for i in 0..height {
                let count = if array[i][j] == 1 {
                    high
                } else {
                    low
                };
                let t = git2::Time::new(commit_time.to_timespec().sec, 0);
                let s = git2::Signature::new("Paul Dib", "paul.dib@orange.fr", &t).unwrap();
                for _ in 0..count {
                    let commit_id = match previous_commit {
                        None => repo.commit(Some("HEAD"), &s, &s, " ", &repo.find_tree(oid).unwrap(), &[]).unwrap(),
                        Some(commit) => repo.commit(Some("HEAD"), &s, &s, " ", &repo.find_tree(oid).unwrap(), &[&commit]).unwrap(),
                    };
                    previous_commit = repo.find_commit(commit_id).ok();
                }
                commit_time = commit_time + time::Duration::days(1);
            }
        }
    }
}

fn main() {
    let params = read_cli_arguments();
    
    let mut repo = git2::Repository::init(&params[&Parameter::RepositoryPath]).unwrap();
    

    let low = params[&Parameter::LowCommitCount].parse::<i32>().unwrap();
    let high = params[&Parameter::HighCommitCount].parse::<i32>().unwrap();
    
    write_to_repository(&mut repo, low, high, params[&Parameter::Text].clone());
}

