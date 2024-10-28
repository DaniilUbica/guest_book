use std::sync::Arc;
use rocket::{get, State};
use rocket::response::content;
use rocket::response::content::RawHtml;
use rocket_dyn_templates::{context, Template};
use crate::database::read_from_database;
use crate::error::error::Error;
use crate::models::{Comment, User};

#[get("/")]
pub fn index(db_pool: &State<Arc<rayon::ThreadPool>>) -> Template {
    let users = match db_pool.install(|| {
        read_from_database::<User>("users")
    })
    {
        Ok(users) => users,
        Err(err) => { return
        Template::render("error", context! { error_message: err.to_str(), error_concrete_message: err.error_text.clone() }); }
    };

    let comments = match db_pool.install(|| {
        read_from_database::<Comment>("comments")
    })
    {
        Ok(comments) => comments,
        Err(err) => { return
        Template::render("error", context! { error_message: err.to_str(), error_concrete_message: err.error_text.clone() }); }
    };

    Template::render("index", &context! { users: users, comments: comments })
}

#[get("/new_comment")]
pub fn enter_new_comment() -> RawHtml<&'static str> {
    RawHtml(include_str!("../templates/new_comment.html"))
}