use sqlx::mysql::MySqlPoolOptions;

#[derive(Clone)]
pub struct MailingList {
    pub email: String,
}

pub async fn add_email(email: String, database: String) -> Result<String, String>{
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database)
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
            match sqlx::query!(r#"
                               INSERT INTO mailing_list (email) VALUES (?)"#, 
                               email)
                .execute(&pool)
                .await {
                    Ok(_) => Ok(format!("Successfully added email!")),
                    Err(err) => Err(format!("Error adding email to database: {}", err)),
                }
            }
    }
}

pub async fn remove_email(email: String, database: String) -> Result<String, String>{
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database)
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
