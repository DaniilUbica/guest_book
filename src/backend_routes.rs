use std::sync::Arc;
use rocket::{post, State};
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket_dyn_templates::Template;
use crate::database::{get_record_id_from_info, read_from_database, write_to_database};
use crate::error::error::{ApplicationError, DatabaseError, Error};
use crate::models::{Comment, User};

#[derive(Debug, Deserialize)]
struct CommentInput {
    mail: String,
    login: String,
    comment: String,
}

#[post("/user", format = "json", data = "<new_user>")]
pub fn api_add_user(new_user: Json<User>, db_pool: &State<Arc<rayon::ThreadPool>>) -> Result<(), Error> {
    let user = new_user.into_inner();
    let fixed_db_pool = db_pool.inner();

    fixed_db_pool.install(|| {
        write_to_database(&user)
    })
}

#[post("/new_comment", format = "json", data = "<new_comment>")]
pub fn api_add_comment(new_comment: Json<Comment>, db_pool: &State<Arc<rayon::ThreadPool>>) -> Result<(), Error> {
    let comment = new_comment.into_inner();
    let fixed_db_pool = db_pool.inner();

    fixed_db_pool.install(|| {
        write_to_database(&comment)
    })
}

#[post("/new_comment", format = "json", data = "<new_comment_input>")]
pub fn input_new_comment(new_comment_input: Json<CommentInput>, db_pool: &State<Arc<rayon::ThreadPool>>) -> Result<(), Error> {
    let input_comment = new_comment_input.into_inner();
    let fixed_db_pool = db_pool.inner();

    fixed_db_pool.install(|| {
        let users = read_from_database::<User>("users")?;

        if let Some(user) = users.iter().find(|user| user.mail == input_comment.mail) {
            let comment = Comment {
                from: user.clone(),
                comment: input_comment.comment
            };

            write_to_database(&comment)?;
        }
        else {
            let user = User {
                mail: input_comment.mail,
                login: input_comment.login
            };
            let comment = Comment {
                from: user.clone(),
                comment: input_comment.comment
            };

            write_to_database(&user)?;
            write_to_database(&comment)?;
        }
        Ok(())
    })
}