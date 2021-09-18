use crate::core::path::get_namespaces_file_path;
use crate::utils::errors::{invalid_input_error, not_found_error, std_error};
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::fs;
use std::io::Error;
use std::path::Path;
use tabled::Tabled;

mod methods;

pub use methods::*;

#[derive(Serialize, Deserialize, Tabled, Debug, Clone)]
pub struct RemoteRepoNamespace {
    pub name: String,
    pub base_url: String,
    pub requires_authorization: bool,
}

pub fn parse_to_raw_url(route: String) -> Result<String, Error> {
    let regex = std_error(Regex::new(r"^[\w-]+"))?;
    let namespace_name = regex.find(&route);

    if let Some(namespace_name) = namespace_name {
        if let Some(namespace) = get_namespace(namespace_name.as_str())? {
            let raw_url = regex.replace(&route, namespace.base_url).to_string();
            return Ok(raw_url);
        } else {
            return Err(not_found_error(&format!(
                "Not found namespace named as \"{}\".",
                namespace_name.as_str()
            )));
        }
    }

    Err(invalid_input_error("Invalid namespace syntax."))
}

pub fn get_namespace(namespace_name: &str) -> Result<Option<RemoteRepoNamespace>, Error> {
    let repos = get_saved_namespaces()?;
    Ok(repos
        .into_iter()
        .find(|namespace| namespace.name == namespace_name))
}

pub fn get_saved_namespaces() -> Result<Vec<RemoteRepoNamespace>, Error> {
    let remote_repos_filename = get_namespaces_file_path()?;
    let current_repos_json = fs::read_to_string(&remote_repos_filename)?;
    std_error(from_str(&current_repos_json))
}

fn update_namespace_file(repos: Vec<RemoteRepoNamespace>) -> Result<(), Error> {
    fs::write(
        get_namespaces_file_path()?,
        std_error(to_string_pretty(&repos))?,
    )?;
    Ok(())
}

pub fn create_namespaces_file() -> Result<(), Error> {
    let namespaces_file_not_exists = !Path::new(&get_namespaces_file_path()?).exists();
    if namespaces_file_not_exists {
        let initial_content = vec![get_std_tools_namespace()];
        fs::write(
            get_namespaces_file_path()?,
            std_error(to_string_pretty(&initial_content))?,
        )?;
    }

    Ok(())
}

fn get_std_tools_namespace() -> RemoteRepoNamespace {
    RemoteRepoNamespace {
        name: "std-tools".to_string(),
        base_url: "https://templo-std-tools.herokuapp.com/v1".to_string(),
        requires_authorization: false,
    }
}