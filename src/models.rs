use postgres::{Client, NoTls, Error as PgError};
use rocket::serde::{Deserialize, Serialize};
use crate::error::error::{ApplicationError, DatabaseError, Error};
use crate::database::{DB_HOST, DB_USER, DB_NAME, get_record_id_from_info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub mail: String,
    pub login: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub from: User,
    pub comment: String,
}

pub trait Writable {
    fn save_to_database(&self, client: &mut Client) -> Result<(), Error>;
}

pub trait Readable {
    fn from_database(client: &mut Client, id: i32) -> Result<Self, Error> where Self: Sized;
    fn from_row(row: &postgres::Row) -> Result<Self, PgError> where Self: Sized;
}

impl Writable for User {
    fn save_to_database(&self, client: &mut Client) -> Result<(), Error> {
        client.execute(
            "INSERT INTO users (mail, login) VALUES ($1, $2)",
            &[&self.mail, &self.login],
        ).unwrap();
        Ok(())
    }
}

impl Writable for Comment {
    fn save_to_database(&self, client: &mut Client) -> Result<(), Error> {
        let user = &self.from;
        let id = get_record_id_from_info("user_id", "Users", &vec!["mail", "login"], &vec![&user.mail, &user.login])?;

        client.execute(
            "INSERT INTO comments (user_id, comment) VALUES ($1, $2)",
            &[&id, &self.comment],
        )?;

        Ok(())
    }
}

impl Readable for User {
    fn from_database(client: &mut Client, id: i32) -> Result<Self, Error> {
        let row = client.query_one(
            "SELECT mail, login FROM users WHERE user_id = $1",
            &[&id],
        )?;
        User::from_row(&row).map_err(|e| Error::new(ApplicationError::DatabaseError(DatabaseError::Error("Error reading user from db".to_string())), e.to_string()))
    }

    fn from_row(row: &postgres::Row) -> Result<Self, PgError> {
        Ok(User {
            mail: row.get("mail"),
            login: row.get("login"),
        })
    }
}

impl Readable for Comment {
    fn from_database(client: &mut Client, id: i32) -> Result<Self, Error> {
        let row = client.query_one(
            "SELECT u.mail, u.login, c.comment FROM comments c JOIN users u ON c.user_id = u.id WHERE c.user_id = $1",
            &[&id],
        )?;
        Comment::from_row(&row).map_err(|e| Error::new(ApplicationError::DatabaseError(DatabaseError::Error("Error reading comment from db".to_string())), e.to_string()))
    }

    // todo: remove client creation here
    fn from_row(row: &postgres::Row) -> Result<Self, PgError> {
        let mut client = Client::connect(&format!("host={DB_HOST} user={DB_USER} dbname={DB_NAME}"), NoTls)?;
        let from = User::from_database(&mut client, row.get("user_id")).unwrap();

        Ok(Comment {
            from: from,
            comment: row.get("comment")
        })
    }
}