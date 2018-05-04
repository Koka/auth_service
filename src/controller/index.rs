#[get("/")]
pub fn handler() -> &'static str {
  "Rust authz service"
}
