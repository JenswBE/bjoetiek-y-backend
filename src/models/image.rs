use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ThumbnailRequest {
    /// UUID of the owner (product, category, ...)
    pub id: uuid::Uuid,

    /// Max width of thumbnail
    pub width: i32,

    /// Max height of thumbnail
    pub height: i32,

    /// Crop image to fill width and height completely
    pub fill: bool,
}
