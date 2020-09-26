use serde::{Deserialize};
use warp::{
    http::{Response},
    Filter,
};
use std::process::Command;
use std::process::Stdio;
// use std::env;
use std::path::Path;
// use std::io::{self, Write};

#[derive(Deserialize)]
struct GitRepo{
    repo_url: String,
    repo_name: String,
}

#[tokio::main]
async fn main(){

    let first_commit_endpoint = warp::get()
                        .and(warp::path("first-commit"))
                        .and(warp::query::<GitRepo>())
                        .map(|git_repo: GitRepo| {
                            let mut s1 = "/tmp/".to_owned();
                            s1.push_str(&git_repo.repo_name.to_owned());
                            let wd = Path::new(&s1);
                            Command::new("git")
                                    .arg("clone")
                                    .arg(git_repo.repo_url)
                                    .arg(wd)
                                    .output();

                            let mut git_log_child = Command::new("git")
                                                        .current_dir(wd)
                                                        .arg("log")
                                                        .arg("--reverse")
                                                        .stdout(Stdio::piped())
                                                        .spawn()
                                                        .expect("Failed to execute command");
                            let git_log_child_out = git_log_child.stdout.expect("failed to open 'echo' stdout");
                            let sed_child = Command::new("head")
                                .arg("-3")
                                .stdin(Stdio::from(git_log_child_out))
                                .stdout(Stdio::piped())
                                .spawn()
                                .expect("failed to start 'sed'");
                            let sed_out = sed_child
                                .wait_with_output()
                                .expect("failed to wait on 'sed'");
                            let encoded = String::from_utf8_lossy(sed_out.stdout.as_slice());
                            Response::builder().body(format!("{}", encoded))
                        });
    
    warp::serve(first_commit_endpoint)
        .run(([127,0,0,1], 3030))
        .await;
}