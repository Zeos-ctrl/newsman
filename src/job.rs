use sqlx::mysql::MySqlPoolOptions;
use log::debug;
use chrono::Utc;
use lettre::transport::smtp::authentication::Credentials; 
use lettre::{SmtpTransport, Transport};
use lettre::message::{header::ContentType, Message};
use tokio::time::{interval, Duration};

use crate::Config;
use crate::emails::MailingList;

#[derive(Clone)]
struct Job {
    newsletter: String,
    time: i64,
}

pub async fn add_job(newsletter: String, delay: i64) -> Result<String, String>{
    let config: Config = Config::load_config().unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.url)
        .await
        .expect("Cannot connect to database!");

    match sqlx::query!(r#"INSERT INTO jobs (newsletter, time) VALUES (?, ?)"#,
        newsletter,
        delay)
        .execute(&pool)
        .await {
            Ok(_) => Ok(format!("successfully added job")),
            Err(err) => Err(format!("error adding job: {}", err))
        }
}

pub async fn remove_job(newsletter: String) -> Result<String, String>{
    let config: Config = Config::load_config().unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.url)
        .await
        .expect("Cannot connect to database!");

    match sqlx::query!(r#"DELETE FROM jobs WHERE newsletter = (?)"#,newsletter)
        .execute(&pool)
        .await {
            Ok(_) => Ok(format!("successfully removed job")),
            Err(err) => Err(format!("error removing job: {}", err))
        }
}

pub fn execute_job(newsletter: String, clients: &Vec<MailingList>) -> Result<(), ()> {
    let config: Config = Config::load_config().unwrap();
    
    let creds = Credentials::new(config.smtp_username, config.smtp_password);
    let mailer = SmtpTransport::relay(&config.relay) 
        .unwrap() 
        .credentials(creds) 
        .build(); 
    for client in clients {
        let email = Message::builder() 
            .from(config.sender.clone().parse().unwrap()) 
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

pub async fn execute_daemon(){
    let config: Config = Config::load_config().unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.url)
        .await
        .expect("Cannot connect to database!");

    tokio::spawn(async move{
        let mut interval = interval(Duration::from_secs(config.interval * 60));
        interval.tick().await; // first tick fires immediately, ignore it
        loop {
            interval.tick().await;

            let jobs_list: Result<Vec<Job>, sqlx::Error> = sqlx::query_as!(Job,"SELECT * FROM jobs")
                .fetch_all(&pool)
                .await;

            let clients: Vec<MailingList> = sqlx::query_as!(MailingList, "SELECT * from mailing_list")
                .fetch_all(&pool)
                .await
                .unwrap();

            match jobs_list{
                    Ok(jobs) => {
                        for newsletter in jobs {
                            if compare_time(newsletter.time){
                                execute_job(newsletter.newsletter.clone(), &clients).unwrap();
                                remove_job(newsletter.newsletter).await.unwrap();
                            }
                        };
                    },
                    Err(err) => debug!("Error getting jobs from database: {}", err)
                }
            }
    });
}

fn compare_time(time: i64) -> bool{
    let start_time = Utc::now().timestamp();
    if start_time - time < 0 {
        true
    } else {
        false
    } 
}

#[cfg(test)]
mod tests {
    use lettre::transport::smtp::authentication::Credentials; 
    use lettre::{SmtpTransport, Transport};
    use lettre::message::{header::ContentType, Message};

    use crate::config::Config;

    #[tokio::test]
    async fn new_job() {
        let config: Config = Config::load_config().unwrap();

        let creds = Credentials::new(config.smtp_username, config.smtp_password);
        let mailer = SmtpTransport::relay(&config.relay) 
            .expect("Relay Error") 
            .credentials(creds) 
            .build(); 

        let email = Message::builder() 
            .from(config.sender.clone().parse().unwrap()) 
            .to(config.sender.clone().parse().unwrap()) 
            .subject("Newsletter") 
            .header(ContentType::TEXT_PLAIN)
            .body(String::from("Newsletter test"))
            .unwrap(); 

        match mailer.send(&email) { 
              Ok(_) => assert!(true), 
              Err(e) => panic!("Could not send email: {:?}", e), 
            }
    }
}
