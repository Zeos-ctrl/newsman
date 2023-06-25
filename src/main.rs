pub mod config;
pub mod emails;
pub mod job;

extern crate daemonize;

use chrono::Utc;
use daemonize::Daemonize;
use std::fs::{File, create_dir};
use std::path::{Path, PathBuf};
use env_logger::Builder;
use clap::Parser;
use log::{debug, LevelFilter};

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

    /// Starts a tokio server that automatically does jobs when the time comes, -e
    #[arg(short)]
    execute: Option<bool>,

    /// Run the program as a daemon, -d
    #[arg(short)]
    daemon: Option<bool>,

    /// Turn debugging information on
    #[arg(long, action = clap::ArgAction::Count)]
    debug: u8,
}

async fn parse_cli(cli: Args) -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    let database: String = Config::load_config().unwrap().url;

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
        let output: Result<String, String> = emails::add_email(email.to_string(), database.clone())
            .await;

        match output {
            Ok(output) => println!("{}", output),
            Err(err) => println!("{}", err)
        }
    }

    if let Some(email) = cli.remove_email.as_deref() {
        debug!("{}", email);
        let output: Result<String, String> = emails::remove_email(email.to_string(), database.clone())
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
        let output: Result<String, String> = job::add_job(job.to_string(), delay)
            .await;

        match output {
            Ok(output) => println!("{}", output),
            Err(err) => println!("{}", err)
        }
    }

    if let Some(job) = cli.job.as_deref() {
        debug!("Unassinging job: {}", &job);
        let output: Result<String, String> = job::remove_job(job.to_string())
            .await;

        match output {
            Ok(output) => println!("{}", output),
            Err(err) => println!("{}", err)
        }
    }

    Ok(())

}

fn first_time_setup() -> anyhow::Result<()>{
    // setup wizard construct a config and save to file
    let mut default_config: Config = Config::default();

    let home: PathBuf = dirs::home_dir().expect("cannot get home dir");
    match std::fs::read_dir(format!("{}/.config", &home.display())) {
        Ok(_) => debug!(".config exists continuing setup"),
        Err(_) => {
            debug!(".config not found creating it now");
            std::fs::create_dir(format!("{}/.config", &home.display()))
                .expect("failed to make .config")
        }
    }

    std::fs::create_dir(format!("{}/.config/newsman", &home.display())).expect("failed to make newsman dir");
    std::fs::create_dir(format!("{}/.config/newsman/newsletters", &home.display())).expect("failed to make newsletter dir");

    println!("Welcome to Newsman! There are no config files detected so this wizard will help you construct them.");
    println!("Would you like to use the default settings and edit them manually later? Y/n");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to read input");
    debug!("{}", &input);
    input.pop();

    if input.eq("Y") || input.eq("y") {
        println!("Using default config, this can be edited in ~/.config/newsman/");
        default_config.save_config();
        Ok(())
    } else {
        println!("Enter the database url, e.g, mysql://root:password@localhost/newsman");
        let mut url = String::new();
        std::io::stdin().read_line(&mut url).expect("failed to read input");
        url.pop();
        default_config.set_url(url);

        println!("Enter your smtp_username, e.g, example@example.com");
        let mut smtp_username = String::new();
        std::io::stdin().read_line(&mut smtp_username).expect("failed to read input");
        smtp_username.pop();
        default_config.set_smtp_username(smtp_username);

        println!("Enter your smtp_password, e.g, 12345");
        let mut smtp_password = String::new();
        std::io::stdin().read_line(&mut smtp_password).expect("failed to read input");
        smtp_password.pop();
        default_config.set_smtp_password(smtp_password);

        println!("Enter your smtp relay, e.g, mail.example.com");
        let mut relay = String::new();
        std::io::stdin().read_line(&mut relay).expect("failed to read input");
        relay.pop();
        default_config.set_relay(relay);

        println!("Enter the time interval that the server should check for jobs to do in minuites");
        let mut interval = String::new();
        std::io::stdin().read_line(&mut interval).expect("failed to read input");
        let x: u64 = interval.trim().parse().expect("Input not an integer");
        default_config.set_interval(x);

        println!("These are your settings:\n{:?}", &default_config);
        default_config.save_config();
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let cli = Args::parse();
    let mut builder = Builder::from_default_env();

    let home: PathBuf = dirs::home_dir().expect("Cannot find home dir");

    match std::fs::read_to_string(format!("{}/.config/newsman/newsman.toml", home.display())) {
        Ok(_) => debug!("Configs are correct and made"),
        Err(_) => first_time_setup().unwrap(),
    }

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => {
           builder
               .filter(None, LevelFilter::Debug)
               .init()
        },
        _ => println!("Don't be crazy"),
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
