// Later yew endpoint
#[get("/")]
pub fn index() -> &'static str {
    "Hello from kyward!"
}
