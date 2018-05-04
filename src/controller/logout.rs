use rocket::http::{Cookie, Cookies};
use super::RemoteIP;

#[post("/authz/logout")]
pub fn handler(mut cookies: Cookies, ip: RemoteIP) -> () {
  info!("Logout from {:?}", ip.0);

  let cookie = Cookie::build("token", "")
    .path("/")
    .secure(false)
    .http_only(false)
    .finish();

  cookies.add(cookie);
}
