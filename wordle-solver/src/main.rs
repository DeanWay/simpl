use clap::Command;
use std::error::Error;
mod commands;
mod constraint;
mod game;
mod io;
mod solver;
mod solver_strategy;
mod types;
mod util;

fn cli() -> Command<'static> {
    Command::new("wordle-solver")
        .about("a wordle game an solving strategies")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("game").about("play game"))
        .subcommand(Command::new("solver").about("run solver analysis"))
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("game", _sub_matches)) => commands::game::run_cli_game(),
        Some(("solver", _sub_matches)) => {
            commands::solver::run_solver();
        }
        _ => unreachable!(),
    };
    Ok(())
}
