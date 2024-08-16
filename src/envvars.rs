use std::env;
use std::collections::{HashMap, HashSet};

fn get_env_var(key: &String) -> String {
    match env::var(key){
        Ok(value) => value,
        Err(_) => "".to_string()
    }
}

pub fn get_envs(requirements: HashSet<String>) -> Result<HashMap<String, String>, String> {
    let mut envs = HashMap::new();

    for requirement in &requirements {
        if !requirement.starts_with("env.") {
            return Err(format!("There is an invalid value in the template. The value is {}. Variables must start with .env.", requirement));
        }

        let key = requirement.replace("env.", "");

        envs.insert(key.to_string(), get_env_var(&key));
    }

    Ok(envs)
}

