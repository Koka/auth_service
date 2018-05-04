use super::{RemoteIP};
use rocket::request::{Form};
use std::error::Error;

use rocket::response::status::{Custom};
use rocket::http::Status;
use jwt::{encode, decode, Validation, Header};

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
pub struct TokenRequest {
  grant_type: String,
  assertion: String
}

#[post("/token", data = "<form>")]
pub fn handler(form: Form<TokenRequest>, ip: RemoteIP) -> Result<Custom<String>, Box<Error>> {
  let req = form.get();

  let token = req.assertion.replace("Bearer ", "");

  info!("TOKEN ??? {:?}", token);

  let jwt_grant = "urn:ietf:params:oauth:grant-type:jwt-bearer";

  let validation = Validation::default();
  let decoded = decode::<JwtClaims>(&token, "BbZJjyoXAdr8BUZuiKKARWimKfrSmQ6fv8kZ7Offf".as_ref(), &validation);

  info!("TOKEN {:?}", decoded);

  if !decoded.is_ok() {
    return Ok(Custom(Status::Forbidden, "Invalid token".to_owned()));
  }

  let claims = decoded.unwrap().claims;

  let token = encode(&Header::default(), &claims, "BbZJjyoXAdr8BUZuiKKARWimKfrSmQ6fv8kZ7Offf".as_ref())?;

  let response = json!({
    "access_token": token,
    "token_type": "bearer",
    "expires_in": 3600
  });

  Ok(Custom(Status::Ok, response.to_string()))
}
