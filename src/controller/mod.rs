pub mod logout;
pub mod challenge;
pub mod app_setup;
pub mod token;
pub mod index;

use rocket::request::{self, Request, FromRequest};
use rocket::{Outcome, State};
use rocket::http::Status;

use std::net::IpAddr;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager};
use postgres::Connection;

use std::ops::Deref;

pub struct RemoteIP(Option<IpAddr>);

impl<'a, 'r> FromRequest<'a, 'r> for RemoteIP {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
    let ip = request.client_ip();
    return Outcome::Success(RemoteIP(ip));
  }
}

pub struct DB(pub PooledConnection<PostgresConnectionManager>);

impl<'a, 'r> FromRequest<'a, 'r> for DB {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
    let pool = request.guard::<State<Pool<PostgresConnectionManager>>>()?;
    match pool.get() {
      Ok(conn) => Outcome::Success(DB(conn)),
      Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
    }
  }
}

impl Deref for DB {
  type Target = Connection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
