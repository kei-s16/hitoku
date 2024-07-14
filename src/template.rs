use minijinja::{Environment, context};

use crate::envvars;

pub fn generate_content(body: &str) -> Result<String, String> {
    let mut jinja = Environment::new();
    jinja.add_template("template", body).unwrap();
    let tmpl = jinja.get_template("template").unwrap();

    let requirement_env_vars = tmpl.undeclared_variables(true);

    let contexts = match envvars::get_envs(requirement_env_vars) {
        Ok(contexts) => contexts,
        Err(message) => return Err(message)
    };

    Ok(tmpl.render(context!(env => contexts)).unwrap())
}

