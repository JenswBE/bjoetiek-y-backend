use std::path::PathBuf;

use actix::{Actor, Handler, Message, SyncContext};
use failure::{format_err, Error};
use image::imageops::FilterType;
use uuid::Uuid;

const SAMPLING_FILTER: FilterType = FilterType::Lanczos3;
const THUMBNAIL_SPECS: [ThumbnailSpec; 1] = thumbnail_specs();
const fn thumbnail_specs() -> [ThumbnailSpec; 1] {
    [
        // General admin
        ThumbnailSpec {
            width: 100,
            height: 100,
            fill: false,
        },
    ]
}

pub struct ImageActor {
    path: PathBuf,
}

impl Actor for ImageActor {
    type Context = SyncContext<Self>;
}

impl ImageActor {
    pub fn new(images_path: PathBuf) -> Self {
        if !images_path.exists() {
            panic!("Provided images path does not exist")
        }

        Self { path: images_path }
    }

    /// Generates an image path based on an image ID
    pub fn get_image_path(&self, id: Uuid) -> PathBuf {
        self.path.join(id.to_string()).with_extension("png")
    }
}

pub struct UploadImage {
    pub id: Uuid, // ID of the owner (product, manufacturer, ...)
    pub data: Vec<u8>,
}

impl Message for UploadImage {
    type Result = Result<(), Error>;
}

impl Handler<UploadImage> for ImageActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: UploadImage, context: &mut Self::Context) -> Self::Result {
        // Load image and write as png
        let path = self.get_image_path(msg.id).to_string_lossy().to_string();
        let image = image::load_from_memory(&msg.data).map_err(Error::from)?;
        image.save(path).map_err(Error::from)?;

        // Request thumbnail creation
        context
            .address()
            .try_send(GenerateThumbnails { id: msg.id })
            .map_err(|_| format_err!("Failed to request thumbnail generation for {}", msg.id))
    }
}

struct ThumbnailSpec {
    /// Max width of thumbnail
    pub width: u32,

    /// Max height of thumbnail
    pub height: u32,

    /// Crop image to fill width and height completely
    pub fill: bool,
}

pub struct GenerateThumbnails {
    pub id: Uuid,
}

impl Message for GenerateThumbnails {
    type Result = Result<(), Error>;
}

impl Handler<GenerateThumbnails> for ImageActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: GenerateThumbnails, _: &mut Self::Context) -> Self::Result {
        let image_path = self.get_image_path(msg.id);
        for spec in &THUMBNAIL_SPECS {
            // Load image from file
            let img = image::open(&image_path).map_err(|e| {
                log::error!("Failed to generate thumbnail for {:?}: {}", image_path, e);
                return Error::from(e);
            })?;

            // Generate thumbnail
            let thumbnail = if spec.fill {
                img.resize_to_fill(spec.width, spec.height, SAMPLING_FILTER)
            } else {
                img.resize(spec.width, spec.height, SAMPLING_FILTER)
            };

            // Build filename
            let fill_fit = if spec.fill { "fill" } else { "fit" };
            let thumbnail_name = format!(
                "{}-{}-{}-{}.{}",
                msg.id,
                spec.width,
                spec.height,
                fill_fit,
                image_path.extension().unwrap().to_string_lossy(),
            );
            let thumbnail_path = image_path.with_file_name(thumbnail_name);

            // Write thumbnail as png
            thumbnail.save(&thumbnail_path).map_err(|e| {
                log::error!(
                    "Failed to write thumbnail to file {:?}: {}",
                    thumbnail_path,
                    e
                );
                Error::from(e)
            })?;
        }

        Ok(())
    }
}
