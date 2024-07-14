use std::env;
use std::collections::{HashMap, HashSet};

fn get_env_var(key: &str) -> String {
    match env::var(key){
        Ok(value) => value,
        Err(_) => "".to_string()
    }
}

pub fn get_envs(requirements: HashSet<String>) -> Result<HashMap<String, String>, String> {
    let mut envs = HashMap::new();

    for value in &requirements {
        if !value.starts_with("env.") {
            return Err(format!("There is an invalid value in the template. The value is {}. Variables must start with .env.", value));
        }

        envs.insert(value.to_string(), get_env_var(value));
    }

    Ok(envs)
}

