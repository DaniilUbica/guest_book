pub mod error {
    use postgres;
    use rocket::Responder;

    #[derive(Clone, Debug, Responder)]
    pub enum DatabaseError {
        Error(String),
        UnknownError(String)
    }

    #[derive(Clone, Debug, Responder)]
    pub enum ApplicationError {
        DatabaseError(DatabaseError)
    }

    #[derive(Debug, Responder)]
    pub struct Error {
        pub error: ApplicationError,
        #[response(ignore)]
        pub error_text: String
    }

    impl Error {
        pub fn new(err: ApplicationError, txt: String) -> Error {
            Error {
                error: err,
                error_text: txt
            }
        }

        pub fn to_str(&self) -> String {
            match &self.error {
                ApplicationError::DatabaseError(db_err) => match db_err.clone()
                {
                    DatabaseError::Error(e) => e,
                    DatabaseError::UnknownError(e) => e
                }
            }
        }
    }

    impl From<postgres::Error> for Error {
        fn from(value: postgres::Error) -> Self {
            let db_error = value.as_db_error();
            if let Some(db_error) = db_error {
                Error::new(ApplicationError::DatabaseError(DatabaseError::Error("Error working with database".to_string())), db_error.to_string())
            }
            else {
                Error::new(ApplicationError::DatabaseError(DatabaseError::UnknownError("Error working with database".to_string())), String::from("An unknown error occured"))
            }
        }
    }
}