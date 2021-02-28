use std::fs;
use std::path::PathBuf;

use actix::{Actor, Handler, Message, SyncContext};
use failure::{format_err, Error};
use image::imageops::FilterType;
use uuid::Uuid;

const SAMPLING_FILTER: FilterType = FilterType::Lanczos3;
const THUMBNAIL_SPECS: [ThumbnailSpec; 4] = thumbnail_specs();
const fn thumbnail_specs() -> [ThumbnailSpec; 4] {
    [
        // Products overview
        ThumbnailSpec {
            width: 400,
            height: 400,
            fill: false,
        },
        // Product details
        ThumbnailSpec {
            width: 550,
            height: 550,
            fill: false,
        },
        // Manufacturer logo
        ThumbnailSpec {
            width: 150,
            height: 150,
            fill: false,
        },
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

pub struct DeleteImage {
    pub id: Uuid,
}

impl Message for DeleteImage {
    type Result = Result<(), Error>;
}

impl Handler<DeleteImage> for ImageActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: DeleteImage, _: &mut Self::Context) -> Self::Result {
        // Get directory entries
        let entries = fs::read_dir(&self.path).map_err(|e| {
            log::warn!("Failed to read images directory to delete image: {}", e);
            Error::from(e)
        })?;

        // Find and delete image and thumbnails
        for entry in entries {
            // Skip entry on error
            if entry.is_err() {
                log::info!(
                    "Failed to read images directory entry: {}",
                    entry.unwrap_err()
                );
                continue;
            }

            // Check if related
            let entry = entry.unwrap();
            let is_related = entry
                .file_name()
                .to_string_lossy()
                .starts_with(&msg.id.to_string());

            // Delete file if image or thumbnail
            if is_related {
                if let Err(e) = fs::remove_file(entry.path()) {
                    log::warn!("Failed to delete image at {:?}: {}", entry.path(), e);
                }
            }
        }
        Ok(())
    }
}
