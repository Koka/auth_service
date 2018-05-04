use super::{RemoteIP, DB};
use super::super::Config;

use rocket::State;
use jwt::{encode, Header};
use rocket_contrib::Json;
use std::error::Error;

use rocket::response::status::{Custom};
use rocket::http::Status;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AppSetupRequest {
  orgCode: String,

}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct JwtClaims {
  clientId: String,
  registerCode: String
}


#[post("/authz/appsetup", format = "application/json", data = "<request>")]
pub fn handler(request: Json<AppSetupRequest>, conn: DB, ip: RemoteIP, cfg: State<Config>) -> Result<Custom<String>, Box<Error>> {
  let mut org_code = request.0.orgCode.split("/");
  let org_id = org_code.next();

  if org_id.is_none() {
    return Ok(Custom(Status::NotFound, "Org is not specified on app setup".to_owned()));
  }

  let org_id = org_id.unwrap();

  let register_code = org_code.next();

  if register_code.is_none() {
    return Ok(Custom(Status::NotFound, format!("Register code is not specified on app setup, org = {}", org_id).to_owned()));
  }

  let register_code = register_code.unwrap();

  let register = conn.query("
    SELECT l.mnemonic AS location, r.*
      FROM registers r
    INNER JOIN organizations o ON o.id = r.org_id
    INNER JOIN locations l ON l.id = r.location_id
    WHERE o.namespace = $1 AND r.mnemonic = $2
  ", &[&org_id, &register_code])?;

  let register = register.iter().next();

  info!("DB register {:?}", register);

  if register.is_none() {
    return Ok(Custom(Status::NotFound, "Register not found".to_owned()));
  }

  let register = register.unwrap();

  let enabled: bool = register.get("enabled");

  if !enabled {
    return Ok(Custom(Status::Forbidden, "Register is disabled".to_owned()));
  }

  let claims = JwtClaims {
    clientId: org_id.to_owned(),
    registerCode: register_code.to_owned()
  };

  let token = encode(&Header::default(), &claims, cfg.jwt_secret.as_ref())?;

  let location: String = register.get("location");
  let name: String = register.get("name");

  let response = json!({
    "token": token,
    "orgId": org_id,
    "location": location,
    "name": name,
    "registerCode": register_code,
  });

  Ok(Custom(Status::Ok, response.to_string()))
}
