use rocket::http::ContentType;

#[derive(RustEmbed)]
#[folder = "../kyward-ui/dist"]
struct Asset;

#[get("/")]
pub fn index() -> (ContentType, String) {
    let index_html = &Asset::get("index.html").unwrap();
    let data = std::str::from_utf8(index_html.data.as_ref());
    return (ContentType::HTML, data.unwrap().to_string())
}

#[get("/<id>")]
pub fn static_files(id: &str) -> (ContentType, String) {
    let index_html = &Asset::get(id).unwrap();
    let data = std::str::from_utf8(index_html.data.as_ref());
    //TOO: wasm is not utf8 needs fix
    return (
        match id.split_once('.').unwrap() {
            (_, "html") => ContentType::HTML,
            (_, "js") => ContentType::JavaScript,
            (_, "css") => ContentType::CSS,
            (_, "wasm") => ContentType::new("application", "wasm"),
            _ => ContentType::Plain,
        }, 
        data.unwrap().to_string()
    )
}