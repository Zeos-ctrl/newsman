use serde::Deserialize;
use uuid::Uuid;
use sqlx::mysql::MySqlPoolOptions;
use log::debug;

use crate::Config;

#[derive(Clone, Deserialize)]
pub struct MailingList {
    pub token: String,
    pub email: String,
}

fn is_valid_email(email: String) -> bool {
    let parsed = email
        .split("@")
        .collect::<Vec<&str>>();
    if parsed.len() > 1 && parsed.len() < 3{
        let domain = parsed[(parsed.len() -1)]
            .split(".")
            .collect::<Vec<&str>>();
        if domain[0].is_empty() || domain.len() < 2{
            debug!("{:?}", parsed);
            false
        }else {
            debug!("{:?}", parsed);
            true
        }
    } else {
        debug!("{:?}", parsed);
        false
    }
}

pub async fn add_email(email: String) -> Result<String, String>{
    let config: Config = Config::load_config().unwrap();
    let uuid = Uuid::new_v4();
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.url)
        .await
        .expect("Cannot connect to database!");

    let exists = sqlx::query!(r#"
                              SELECT email FROM mailing_list WHERE email = (?)"#,
                              &email)
        .fetch_one(&pool)
        .await;

    match exists {
        Ok(_) => {
           Ok(format!("Email already exists"))
        },
        Err(_) => {
            if is_valid_email(email.clone()) {
            match sqlx::query!(r#"
                               INSERT INTO mailing_list (token, email) VALUES (?,?)"#,
                               uuid.to_string(),
                               email)
                .execute(&pool)
                .await {
                    Ok(_) => Ok(format!("Successfully added email!")),
                    Err(err) => Err(format!("Error adding email to database: {}", err)),
                }
            } else {
                Err(format!("Email is invalid"))
            }
        }
    }
}

pub async fn remove_email(email: String) -> Result<String, String>{
    let config: Config = Config::load_config().unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.url)
        .await
        .expect("Cannot connect to database!");

    let exists = sqlx::query!(r#"
                              SELECT email FROM mailing_list WHERE email = (?)"#,
                              &email)
        .fetch_one(&pool)
        .await;

    match exists {
        Ok(_) => {
            match sqlx::query!(r#"
                               DELETE FROM mailing_list WHERE email = (?)"#,
                               email)
                .execute(&pool)
                .await {
                    Ok(_) => Ok(format!("Successfully removed email!")),
                    Err(err) => Err(format!("Error removing email from database: {}", err)),
                }
        },
        Err(_) => {
            Err(format!("The email {} doesn't exist in the database", &email))
        }
    }
}

pub async fn remove_email_with_token(token: String) -> Result<String, String>{
    let config: Config = Config::load_config().unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.url)
        .await
        .expect("Cannot connect to database!");

    let exists = sqlx::query!(r#"
                              SELECT email FROM mailing_list WHERE token = (?)"#,
                              &token)
        .fetch_one(&pool)
        .await;

    match exists {
        Ok(_) => {
            match sqlx::query!(r#"
                               DELETE FROM mailing_list WHERE token = (?)"#,
                               token)
                .execute(&pool)
                .await {
                    Ok(_) => Ok(format!("Successfully removed email!")),
                    Err(err) => Err(format!("Error removing email from database: {}", err)),
                }
        },
        Err(_) => {
            Err(format!("The email {} doesn't exist in the database", &token))
        }
    }
}

#[cfg(test)]
mod tests {
    use sqlx::mysql::MySqlPoolOptions;

    #[sqlx::test]
    async fn create_connection(){
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect("mysql://root:password@127.0.0.1:3306/newsman")
            .await;
        match pool {
            Ok(_) => assert!(true),
            Err(err) => panic!("ERROR CONNECTING TO DATABASE: {}", err),
        }
    }

    #[sqlx::test]
    async fn add_email(){
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect("mysql://root:password@127.0.0.1:3306/newsman")
            .await
            .unwrap();

        match sqlx::query!(r#"INSERT INTO mailing_list (email) VALUES (?)"#, format!("example@test.com"))
            .execute(&pool)
            .await {
                Ok(_) => assert!(true),
                Err(err) => panic!("ERROR ADDING EMAIL: {}", err),
            }
    }

    #[sqlx::test]
    async fn remove_email(){
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect("mysql://root:password@127.0.0.1:3306/newsman")
            .await
            .unwrap();

        sqlx::query!(r#"INSERT INTO mailing_list (email) VALUES (?)"#, format!("example@test.com"))
            .execute(&pool)
            .await
            .expect("ERROR ADDING TEST EMAIL");

        match sqlx::query!(r#"DELETE FROM mailing_list WHERE email = (?)"#, format!("example@test.com"))
            .execute(&pool)
            .await {
                Ok(_) => assert!(true),
                Err(err) => panic!("ERROR ADDING EMAIL: {}", err),
            }
    }

}
