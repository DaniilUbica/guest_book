#[macro_use] extern crate rocket;
extern crate guest_book;

use std::collections::HashMap;
use std::sync::Arc;
use rocket_dyn_templates::Template;
use rocket::fs::FileServer;
use guest_book::backend_routes::{api_add_comment, api_add_user, input_new_comment};
use guest_book::database::{create_and_use_database, create_table};
use guest_book::frontend_routes::{enter_new_comment, index};

#[launch]
fn rocket() -> _ {
    // // create_and_use_database().unwrap();
    //
    // let mut attributes = HashMap::new();
    // attributes.insert("user_id", "SERIAL PRIMARY KEY");
    // attributes.insert("mail", "TEXT UNIQUE");
    // attributes.insert("login", "TEXT");
    //
    // create_table("Users", attributes).unwrap();
    //
    // let mut attributes = HashMap::new();
    // attributes.insert("comment_id", "SERIAL PRIMARY KEY");
    // attributes.insert("user_id", "SERIAL REFERENCES Users(user_id)");
    // attributes.insert("comment", "TEXT");
    //
    // create_table("Comments", attributes).unwrap();

    let db_pool = Arc::new(rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap());

    rocket::build()
        .manage(db_pool)
        .mount("/api", routes![api_add_user, api_add_comment])
        .mount("/", routes![input_new_comment])
        .mount("/", routes![index, enter_new_comment])
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
}
