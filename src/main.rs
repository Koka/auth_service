#![feature(plugin)]
#![feature(decl_macro)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

extern crate reqwest;

extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

extern crate jsonwebtoken as jwt;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate hyper;

mod controller;

use r2d2::Pool;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};

pub struct Config {
  jwt_secret: &'static str,
  api_root: &'static str
}

fn main() {
  let db_url = option_env!("DB_URL").unwrap_or("postgres://postgres@localhost/pos_dev");

  let manager = PostgresConnectionManager::new(db_url, TlsMode::None).unwrap();
  let pool = Pool::new(manager).unwrap();

  let config = Config {
    jwt_secret: option_env!("JWT_SECRET").unwrap_or("BbZJjyoXAdr8BUZuiKKARWimKfrSmQ6fv8kZ7Offf"),
    api_root: option_env!("EXTERNAL_API_ROOT").unwrap_or("http://api-stage.example.com/v1")
  };

  rocket::ignite()
    .manage(pool)
    .manage(config)
    .mount("/", routes![
      controller::index::handler,
      controller::app_setup::handler,
      controller::logout::handler,
      controller::challenge::handler,
      controller::token::handler
    ])
    .launch();
}
