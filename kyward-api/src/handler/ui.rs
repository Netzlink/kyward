use rocket::http::ContentType;
use rocket::response::status::NotFound;

const UI_ENTRYPOINT: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "../kyward-ui/dist"]
struct Asset;

#[get("/")]
pub fn index() -> Result<(ContentType, Vec<u8>), NotFound<String>> {
    files(UI_ENTRYPOINT)
}

#[get("/<id>")]
pub fn files(id: &str) -> Result<(ContentType, Vec<u8>), NotFound<String>> {
    let id = match id.contains('.') {
        true => id,
        false => UI_ENTRYPOINT,
    };
    let file = (match Asset::get(id) {
        Some(f) => Ok(f),
        None => Err(NotFound(format!(
            "File with name {0} not found, sorry!",
            id
        ))),
    })?;
    return Ok((
        match id.split_once('.').or(None) {
            Some((_, "html")) => ContentType::HTML,
            Some((_, "js")) => ContentType::JavaScript,
            Some((_, "css")) => ContentType::CSS,
            Some((_, "wasm")) => ContentType::new("application", "wasm"),
            _ => ContentType::Plain,
        },
        file.data.as_ref().to_owned(),
    ));
}
