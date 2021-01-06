use std::path::PathBuf;

use actix::{Actor, Handler, Message, SyncContext};
use failure::{format_err, Error};
use uuid::Uuid;

use crate::models::Thumbnail;

const ICC_PROFILE: &'static str = "sRGB";
const THUMBNAILS: [Thumbnail; 2] = thumbnails();
const fn thumbnails() -> [Thumbnail; 2] {
    [
        Thumbnail {
            width: 400,
            height: 100,
            fill: false,
        },
        Thumbnail {
            width: 100,
            height: 400,
            fill: true,
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
        // Load file into VIPS and write as png
        let path = self.get_image_path(msg.id).to_string_lossy().to_string();
        let image = libvips::VipsImage::new_from_buffer(&msg.data, "").map_err(Error::from)?;
        image.image_write_to_file(&path).map_err(Error::from)?;

        // Request thumbnail creation
        context
            .address()
            .try_send(GenerateThumbnails { id: msg.id })
            .map_err(|_| format_err!("Failed to request thumbnail generation for {}", msg.id))
    }
}

pub struct GenerateThumbnails {
    pub id: Uuid,
}

impl Message for GenerateThumbnails {
    type Result = ();
}

impl Handler<GenerateThumbnails> for ImageActor {
    type Result = ();

    fn handle(&mut self, msg: GenerateThumbnails, _: &mut Self::Context) -> Self::Result {
        let image_path = self.get_image_path(msg.id);
        for thumbnail in &THUMBNAILS {
            // Build thumbnail options
            let mut options = libvips::ops::ThumbnailOptions::default();
            options.height = thumbnail.height;
            options.import_profile = ICC_PROFILE.to_string();
            options.export_profile = ICC_PROFILE.to_string();
            if thumbnail.fill {
                options.crop = libvips::ops::Interesting::Centre;
            }

            // Generate thumbnail from file
            let img_path_str = image_path
                .canonicalize()
                .unwrap()
                .to_string_lossy()
                .to_string();
            let image = libvips::ops::thumbnail_with_opts(&img_path_str, thumbnail.width, &options)
                .map_err(|e| {
                    log::error!("Failed to generate thumbnail for {}: {}", img_path_str, e)
                });
            let image = if let Ok(i) = image { i } else { return };

            // Build filename
            let fill_fit = if thumbnail.fill { "fill" } else { "fit" };
            let file_name = format!(
                "{}-{}-{}-{}.{}",
                msg.id,
                thumbnail.width,
                thumbnail.height,
                fill_fit,
                image_path.extension().unwrap().to_string_lossy(),
            );
            let thumbnail_path = image_path
                .with_file_name(file_name)
                .to_string_lossy()
                .to_string();

            // Write thumbnail as png
            image
                .image_write_to_file(&thumbnail_path)
                .map_err(|e| {
                    log::error!(
                        "Failed to write thumbnail to file {}: {}",
                        thumbnail_path,
                        e
                    )
                })
                .ok();
        }
    }
}
