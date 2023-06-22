use timer::Timer;
use sqlx::mysql::MySqlPoolOptions;
use log::debug;
use lettre::transport::smtp::authentication::Credentials; 
use lettre::{SmtpTransport, Transport};
use lettre::message::{header::ContentType, Message};

use crate::Config;
use crate::emails::MailingList;

pub async fn new_job(newsletter: String, database: String, delay: chrono::Duration) {
    let config: Config = Config::load_config();
    let newsletter: String = std::fs::read_to_string(format!("{}/{}", config.dir, newsletter))
        .expect("unable to find newsletter");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database)
        .await
        .expect("Cannot connect to database!");

    let clients: Vec<MailingList> = sqlx::query_as!(MailingList,"SELECT * FROM mailing_list")
        .fetch_all(&pool)
        .await
        .expect("Cannot get mailing list");

    Timer::new()
        .schedule_with_delay(delay, move ||{
            execute_job(config.sender.clone(), newsletter.clone(), clients.clone()).unwrap();
        });
}

pub fn execute_job(sender: String, newsletter: String, clients: Vec<MailingList>) -> Result<(), ()> {
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

#[cfg(test)]
mod tests {
    use lettre::transport::smtp::authentication::Credentials; 
    use lettre::{SmtpTransport, Transport};
    use lettre::message::{header::ContentType, Message};

    use crate::config::Config;

    #[tokio::test]
    async fn new_job() {
        let config: Config = Config::load_config();

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
