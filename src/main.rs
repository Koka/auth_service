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

fn main() {
  let manager = PostgresConnectionManager::new("postgres://postgres@localhost/pos_dev", TlsMode::None).unwrap();
  let pool = Pool::new(manager).unwrap();

  rocket::ignite()
    .manage(pool)
    .mount("/", routes![
      controller::index::handler,
      controller::app_setup::handler,
      controller::logout::handler,
      controller::challenge::handler,
      controller::token::handler
    ])
    .launch();
}
