use anyhow::Result;
use image::DynamicImage;
use nanoid::nanoid;
use std::{
    fs::{self, File},
    io::{Cursor, ErrorKind, Write},
    path::PathBuf,
};
use zip::{write::SimpleFileOptions, ZipWriter};

pub struct ZipBuilder {
    pub zip: ZipWriter<File>,
    pub path: PathBuf,
}

impl ZipBuilder {
    pub fn create() -> Result<Self> {
        let nanoid = nanoid!(4);
        let timestamp = chrono::Local::now().format("%m%d-$H%M").to_string();
        let path = PathBuf::from(format!("./out/{}-{}.zip", timestamp, nanoid));
        let zip_file = match File::create_new(path.clone()) {
            Ok(file) => file,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                fs::create_dir("./out")?;
                File::create_new(path.clone())?
            }
            Err(e) => Err(e)?,
        };
        let zip = ZipWriter::new(zip_file);

        Ok(Self { zip, path })
    }

    pub fn add_png(&mut self, img: DynamicImage, file_dir: String) -> Result<()> {
        let mut buf = Vec::new();
        let mut writer = Cursor::new(&mut buf);
        img.write_to(&mut writer, image::ImageFormat::Png)?;

        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Bzip2)
            .unix_permissions(0o755);
        self.zip.start_file(file_dir, options)?;

        self.zip.write_all(buf.as_slice())?;

        Ok(())
    }
}
