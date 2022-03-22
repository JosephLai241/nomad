//! Push local Git changes upstream.

use std::path::Path;

use anyhow::Result;
use git2::{
    Config, ConfigLevel, Cred, Direction, ProxyOptions, Remote, RemoteCallbacks, RemoteConnection,
    Repository,
};

use crate::errors::NomadError;

use super::utils::get_repo_branch;

pub fn push_commits(repo: &Repository) -> Result<(), NomadError> {
    if let Some(branch_name) = get_repo_branch(&repo) {
        println!("BRANCH NAME: {branch_name}");
        let mut remote = get_remote_name(&branch_name, "origin", &repo)?;

        if let Ok(_) = remote.connect(Direction::Push) {
            // TODO: CALL remote.push() HERE.
            unimplemented!()
        } else {
            println!("\nREMOTE.CONNECT FAILED!\n");

            let mut callbacks = RemoteCallbacks::new();
            callbacks.credentials(|url, username_from_url, allowed_types| {
                println!(
                    "{}\nUSERNAME FROM URL: {:?}\nALLOWED TYPES: {:?}",
                    url, username_from_url, allowed_types
                );

                //let git_config = Config::new()?.open_level(ConfigLevel::Local)?;

                //for entry in &git_config.entries(None)? {
                //let entry = entry.unwrap();
                //println!("{} => {}", entry.name().unwrap(), entry.value().unwrap());
                //}

                //Cred::credential_helper(
                //&git_config.open_level(ConfigLevel::Local)?,
                //url,
                //username_from_url,
                //)
                Cred::ssh_key(
                    username_from_url.unwrap_or("git"),
                    Some(Path::new("/Users/josephlai/.ssh/id_ed25519.pub")),
                    Path::new("/Users/josephlai/.ssh/id_ed25519"),
                    None,
                )
            });

            callbacks.push_transfer_progress(|one, two, three| {
                println!("FIRST: {one}, SECOND: {two}, THIRD: {three}");
            });

            println!("DONE WITH CALLBACKS");
            let mut proxy_options = ProxyOptions::new();
            proxy_options.auto();

            println!("DONE WITH PROXY OPTIONS");

            let mut remote_connection =
                remote.connect_auth(Direction::Push, Some(callbacks), Some(proxy_options))?;

            dbg!(&remote_connection.connected());

            remote_connection.remote().disconnect()?;
            println!("DISCONNECTED!");
        }
    }

    Ok(())
}

/// Get the remote name from the Git repository.
fn get_remote_name<'a>(
    branch_name: &str,
    remote_name: &str,
    repo: &'a Repository,
) -> Result<Remote<'a>, NomadError> {
    match repo.find_remote(&remote_name) {
        Ok(remote) => Ok(remote),
        Err(_) => {
            let remote_url = repo.remote_set_url(&branch_name, &remote_name)?;

            match repo.find_remote(&remote_name) {
                Ok(remote) => Ok(remote),
                Err(error) => {
                    return Err(NomadError::GitError {
                        context: "Could not get the remote name!".to_string(),
                        source: error,
                    })
                }
            }
        }
    }
}
