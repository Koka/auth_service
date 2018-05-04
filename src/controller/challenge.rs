use super::{RemoteIP, DB};
use super::super::Config;

use reqwest;

use jwt::{encode, Header};

use std::error::Error;

use rocket::State;
use rocket::response::status::{Custom};
use rocket::http::Status;
use rocket::request::{Form};
use rocket::http::{Cookie, Cookies};

header! { (UsernameHeader, "Username") => [String] }
header! { (PasswordHeader, "Password") => [String] }
header! { (AppIdHeader, "app_id") => [String] }
header! { (SignatureHeader, "Signature") => [String] }
header! { (TimestampHeader, "Timestamp") => [String] }

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct JwtClaims {
  id: String,
  clientId: String,
  fullName: String,
  employeeId: String,
  isManager: bool
}

#[derive(FromForm, Debug)]
struct ChallengeRequest {
  username: String,

  password: String,

  #[form(field = "orgId")]
  org_id: String
}

#[post("/authz/challenge", data = "<form>")]
fn handler(form: Form<ChallengeRequest>, ip: RemoteIP, conn: DB, mut cookies: Cookies, cfg: State<Config>) -> Result<Custom<String>, Box<Error>> {
  info!("Login from {:?}; {:?}", ip.0, form);

  let req = form.get();

  let client = reqwest::Client::new();

  let endpoint = format!("{}/auth/login", cfg.api_root);

  let res = client.post(&endpoint)
    .header(UsernameHeader(req.username.clone()))
    .header(PasswordHeader(req.password.clone()))
    .header(AppIdHeader("123".to_owned()))
    .header(SignatureHeader("BYPASS".to_owned()))
    .header(TimestampHeader("123".to_owned()))
    .send()?;

  info!(".COM responded {:?}", res);

  if !res.status().is_success() {
    return Ok(Custom(Status::Forbidden, ".COM rejected auth request".to_owned()));
  }

  let employee = conn.query("
    SELECT e.mnemonic, e.first_name, e.last_name, e.manager
    FROM employees e
    INNER JOIN organizations o ON o.id = e.org_id
    WHERE o.namespace = $1
      AND e.email = $2
      AND e.disabled = FALSE
    ORDER BY e.id
    LIMIT 1
  ", &[&req.org_id, &req.username])?;

  let employee = employee.iter().next();

  info!("DB employee {:?}", employee);

  if employee.is_none() {
    return Ok(Custom(Status::Forbidden, "Employee not found".to_owned()));
  }

  let employee = employee.unwrap();

  let first_name: String = employee.get("first_name");

  let last_name: String = employee.get("last_name");

  let claims = JwtClaims {
    id: req.username.clone(),
    clientId: req.org_id.clone(),
    fullName: format!("{} {}", first_name, last_name),
    employeeId: employee.get("mnemonic"),
    isManager: employee.get("manager")
  };

  let token = encode(&Header::default(), &claims, cfg.jwt_secret.as_ref())?;

  let cookie = Cookie::build("token", token)
    .path("/")
    .secure(false)
    .http_only(false)
    .finish();

  cookies.add(cookie);

  Ok(Custom(Status::Ok, "".to_owned()))
}
