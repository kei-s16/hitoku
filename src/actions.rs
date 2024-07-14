use seahorse::{ActionResult, ActionError, Context};

use crate::io;
use crate::template;

pub fn generate_action(c: &Context) -> ActionResult {
    let template_path = match c.args.first() {
        Some(path) => path,
        None => return Err(ActionError{message: "Path to the template file is required.".to_string()})
    };

    let output_path = match c.args.get(1) {
        Some(path) => path,
        None => return Err(ActionError{message: "Destination file path is required.".to_string()})
    };

    let template = match io::read_file(template_path) {
        Ok(body) => body,
        Err(message) => return Err(ActionError{message})
    };

    let content = match template::generate_content(&template) {
        Ok(content) => content,
        Err(message) => return Err(ActionError{message})
    };

    match io::write_file(output_path, &content) {
        Ok(_) => Ok(()),
        Err(message) => Err(ActionError{message})
    }
}

pub fn preview_action(c: &Context) -> ActionResult {
    let path = match c.args.first() {
        Some(path) => path,
        None => return Err(ActionError{message: "Path to the template file is required.".to_string()})
    };

    let template = match io::read_file(path) {
        Ok(body) => body,
        Err(message) => return Err(ActionError{message})
    };

    match template::generate_content(&template) {
        Ok(content) => {
            println!("{}", content);
            Ok(())
        },
        Err(message) => Err(ActionError{message})
    }
}

pub fn help_action(c: &Context) {
    c.help();
}

