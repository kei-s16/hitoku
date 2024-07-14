use std::env;
use seahorse::{App, Command};

mod envvars;
mod io;
mod template;
mod actions;

fn generate_command() -> Command {
    Command::new("generate")
        .description("Generates and saves files from templates.")
        .usage("hitoku generate [tempalte] [output]")
        .action_with_result(actions::generate_action)
}

fn preview_command() -> Command {
    Command::new("preview")
        .description("Previews the result of template generation to standard output.")
        .usage("hitoku preview [tempalte]")
        .action_with_result(actions::preview_action)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("hitoku [command] [args] [option]")
        .command(preview_command())
        .command(generate_command())
        .action(actions::help_action);

    match app.run_with_result(args){
        Ok(_) => (),
        Err(e) => println!("ERROR: {}", e.message),
    };
}

