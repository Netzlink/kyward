#[get("/version")]
pub fn version() -> String {
    format!(
        "v1alpha1 ({0})",
        crate_version!()
    )
}