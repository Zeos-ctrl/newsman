pub mod config;
pub mod emails;
pub mod job;
pub mod server;

extern crate daemonize;

use chrono::Utc;
use daemonize::Daemonize;
use std::fs::{File, create_dir};
use std::path::Path;
use env_logger::Builder;
use clap::Parser;
use log::{debug, LevelFilter};

use warp;

use crate::config::Config;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Adds an email to the mailing list, -a [email]
    #[arg(short, value_name = "EMAIL")]
    add_email: Option<String>,
   
    /// Remove email from mailing list, -r [email]
    #[arg(short, value_name = "EMAIL")]
    remove_email: Option<String>,

    /// Starts a mailing job, -j [newsletter name]
    #[arg(short, value_name = "NEWSLETTER NAME")]
    job: Option<String>,

    /// Removes a mailing job, -u [newsletter name]
    #[arg(short, value_name = "NEWSLETTER NAME")]
    unassign_job: Option<String>,

    /// Time for job to be started defaults to 0 Minutes, -t [delay for job]
    #[arg(short, value_name = "TIME")]
    time: Option<i64>,

    /// Subject line for a mailing job, defaults to newsletter, -s [subject]
    #[arg(short, value_name = "SUBJECT")]
    subject: Option<String>,

    /// Starts a tokio server that automatically does jobs when the time comes, -e
    #[arg(short)]
    execute: Option<bool>,

    /// Starts a warp server to handle post requests for email signup, -w
    #[arg(short)]
    warp: Option<bool>,

    /// Run the program as a daemon, -d
    #[arg(short)]
    daemon: Option<bool>,

    /// Turn debugging information on
    #[arg(long, action = clap::ArgAction::Count)]
    debug: u8,
}

async fn parse_cli(cli: Args) -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");

    let delay: i64;

    match cli.time {
        Some(time) => {
            debug!("{}", &time);
            delay = Utc::now().timestamp() + (time * 60);
        },
        None => {
            delay = Utc::now().timestamp();
        }
    }

    if let Some(email) = cli.add_email.as_deref() {
        debug!("{}", email);
        let output: Result<String, String> = emails::add_email(email.to_string())
            .await;

        match output {
            Ok(output) => println!("{}", output),
            Err(err) => println!("{}", err)
        }
    }

    if let Some(email) = cli.remove_email.as_deref() {
        debug!("{}", email);
        let output: Result<String, String> = emails::remove_email(email.to_string())
            .await;

        match output {
            Ok(output) => println!("{}", output),
            Err(err) => println!("{}", err)
        }
    }

    if let Some(true) = cli.execute {
        debug!("executing job server");
        job::execute_daemon()
            .await;
    }

    if let Some(job) = cli.job.as_deref() {
        debug!("Executing job in {:?}s", &delay);
        if let Some(subject) = cli.subject.as_deref() {
            let output: Result<String, String> = job::add_job(job.to_string(), delay, subject.to_string())
                .await;

            match output {
                Ok(output) => println!("{}", output),
                Err(err) => println!("{}", err)
            }
        }

    }

    if let Some(unassign_job) = cli.unassign_job.as_deref() {
        debug!("Unassinging job: {}", &unassign_job);
        let output: Result<String, String> = job::remove_job(unassign_job.to_string())
            .await;

        match output {
            Ok(output) => println!("{}", output),
            Err(err) => println!("{}", err)
        }
    }

    Ok(())

}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let cli = Args::parse();
    let mut builder = Builder::from_default_env();

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => {
           builder
               .filter(None, LevelFilter::Debug)
               .init()
        },
        _ => println!("Don't be crazy"),
    }

    if let Some(true) = cli.warp {
        debug!("spawning warp server...");
        warp::serve(server::construct_route())
            .run(([127, 0, 0, 1], 3600))
            .await;
    }

    if let Some(true) = cli.daemon {
        if ! Path::new("/tmp/newsman").is_dir() { // check if tmp dir doesn't exist 
            create_dir("/tmp/newsman").expect("Cannot write to tmp");
        }

        let stdout = File::create("/tmp/newsman/daemon.out")
            .expect("Maybe file exists");
        let stderr = File::create("/tmp/newsman/daemon.err")
            .expect("Maybe file exists");

        let daemonize = Daemonize::new()
            .pid_file("/tmp/newsman/test.pid")
            .working_directory("/tmp/newsman")
            .stdout(stdout)
            .stderr(stderr)
            .privileged_action(|| "Executed before drop privileges");

        match daemonize.start() {
            Ok(_) => {
                debug!("Success, daemonized");
                parse_cli(cli).await.unwrap()
            },
            Err(e) => debug!("Error, {}", e),
        }
    }else {
        parse_cli(cli).await.unwrap()
    }

    Ok(())
}
