#![macro_use]

#[get("/hello")]
pub fn index() -> String {
  String::from("Hello, world!")
}