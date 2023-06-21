pub mod tests;
pub mod config;

extern crate daemonize;

use daemonize::Daemonize;
use timer::Timer;
use std::fs::{File, create_dir};
use std::path::Path;
use env_logger::Builder;
use sqlx::mysql::MySqlPoolOptions;
use clap::Parser;
use log::{debug, LevelFilter};
use lettre::transport::smtp::authentication::Credentials; 
use lettre::{SmtpTransport, Transport};
use lettre::message::{header::ContentType, Message};

use crate::config::Config;

async fn add_email(email: String, database: String) -> Result<String, String>{
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database)
        .await
        .expect("Cannot connect to database!");

    match sqlx::query!(r#"
                       INSERT INTO mailing_list (email) VALUES (?)"#
                       , email)
        .execute(&pool)
        .await {
            Ok(_) => Ok(format!("Successfully added email!")),
            Err(err) => Err(format!("Error adding email to database: {}", err)),
        }
}

async fn remove_email(email: String, database: String) -> Result<String, String>{
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database)
        .await
        .expect("Cannot connect to database!");

    match sqlx::query!(r#"
                       DELETE FROM mailing_list WHERE email = (?)"#
                       , email)
        .execute(&pool)
        .await {
            Ok(_) => Ok(format!("Successfully removed email!")),
            Err(err) => Err(format!("Error removing email from database: {}", err)),
        }
}

#[derive(Clone)]
struct _MailingList {
    email: String,
}

async fn new_job(newsletter: String, database: String, delay: chrono::Duration) {
    let config: Config = Config::load_config();
    let newsletter: String = std::fs::read_to_string(format!("{}/{}", config.dir, newsletter))
        .expect("unable to find newsletter");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database)
        .await
        .expect("Cannot connect to database!");

    let clients: Vec<_MailingList> = sqlx::query_as!(_MailingList,"SELECT * FROM mailing_list")
        .fetch_all(&pool)
        .await
        .expect("Cannot get mailing list");

    Timer::new()
        .schedule_with_delay(delay, move ||{
            execute_job(config.sender.clone(), newsletter.clone(), clients.clone()).unwrap();
        });
}

fn execute_job(sender: String, newsletter: String, clients: Vec<_MailingList>) -> Result<(), ()> {
    let config: Config = Config::load_config();
    
    let creds = Credentials::new(config.smtp_username, config.smtp_password);
    let mailer = SmtpTransport::relay(&config.relay) 
        .unwrap() 
        .credentials(creds) 
        .build(); 
    for client in clients {
        let email = Message::builder() 
            .from(sender.clone().parse().unwrap()) 
            .to(client.email.parse().unwrap()) 
            .subject("Newsletter") 
            .header(ContentType::TEXT_PLAIN)
            .body(newsletter.clone()) 
            .unwrap(); 
        match mailer.send(&email) { 
              Ok(_) => debug!("Email sent successfully!"), 
              Err(e) => panic!("Could not send email: {:?}", e), 
            }
    }

    Ok(())
}


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

    /// Time for job to be started defaults to 0, -t [delay for job]
    #[arg(short, value_name = "TIME")]
    time: Option<i64>,

    /// Executes all mailing jobs, -e
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
    let database: String = Config::load_config().url;

    let delay: chrono::Duration;

    match cli.time {
        Some(time) => {
            debug!("{}", &time);
            delay = chrono::Duration::seconds(time);
        },
        None => {
            delay = chrono::Duration::seconds(0);
        }
    }

    if let Some(email) = cli.add_email.as_deref() {
        debug!("{}", email);
        add_email(email.to_string(), database.clone()).await.unwrap();
    }

    if let Some(email) = cli.remove_email.as_deref() {
        debug!("{}", email);
        remove_email(email.to_string(), database.clone()).await.unwrap();
    }

    if let Some(_) = cli.execute {
    }

    if let Some(job) = cli.job.as_deref() {
        debug!("Executing job in {:?}s", &delay);
        new_job(job.to_string(), database, delay).await;
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

    if let Some(true) = cli.daemon {
        if ! Path::new("/tmp/newsman").is_dir() { // check if tmp dir doesn't exist 
            create_dir("/tmp/newsman").expect("Cannot write to tmp");
        }

        let stdout = File::create("/tmp/newsman/daemon.out").expect("Maybe file exists");
        let stderr = File::create("/tmp/newsman/daemon.err").expect("Maybe file exists");

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
