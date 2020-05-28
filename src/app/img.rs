use super::Config;
use anyhow::{anyhow, bail, Result};
use image::{self, imageops::FilterType, DynamicImage, GenericImageView};
use mozjpeg::{ColorSpace, Compress, ScanMode};
use std::fs;
use std::io::{prelude::*, BufWriter};
use std::path::Path;

pub struct Img<'a> {
    img: DynamicImage,
    config: &'a Config,
    path: &'a Path,
    file_name: String,
    target_dir: &'a Path,
    target_width: usize,
    target_height: usize,
    buf: Vec<u8>,
}

impl<'a> Img<'a> {
    pub fn open(config: &'a Config, path: &'a Path, target_dir: &'a Path) -> Result<Self> {
        let file_name = path
            .file_name()
            .ok_or(anyhow!("No file name."))?
            .to_string_lossy()
            .to_string();

        if let Some(ext) = path.extension() {
            if !ext.to_string_lossy().as_ref().to_lowercase().eq("jpg")
                && !ext.to_string_lossy().as_ref().to_lowercase().eq("jpeg")
            {
                bail!("{} is not jpeg file.", &file_name);
            }
        }
        let img = image::open(path)?;

        Ok(Img {
            img,
            config,
            path,
            file_name,
            target_dir,
            target_width: 0,
            target_height: 0,
            buf: vec![],
        })
    }

    pub fn sharpen(&mut self) -> Result<()> {
        println!("{}", self.config.sharpness);
        let sharpness = ((self.config.sharpness / i16::MAX) as f32 * i32::MAX as f32) as i32;
        println!("{}", sharpness);
        self.img = self.img.unsharpen(1.0, sharpness);
        Ok(())
    }

    pub fn resize(&mut self) -> Result<()> {
        let width = self.img.width() as usize;
        let height = self.img.height() as usize;

        if width > self.config.max_length || height > self.config.max_length {
            if width > height {
                let ratio: f32 = self.config.max_length as f32 / width as f32;
                self.target_width = self.config.max_length;
                self.target_height = (height as f32 * ratio) as usize;
            } else {
                let ratio: f32 = self.config.max_length as f32 / height as f32;
                self.target_width = (width as f32 * ratio) as usize;
                self.target_height = self.config.max_length;
            }
        } else {
            self.target_width = width;
            self.target_height = height;
        }

        self.buf = self
            .img
            .resize(
                self.target_width as u32,
                self.target_height as u32,
                FilterType::Lanczos3,
            )
            .to_rgb()
            .to_vec();
        Ok(())
    }

    pub fn compress(&mut self) -> Result<()> {
        let mut comp = Compress::new(ColorSpace::JCS_RGB);
        comp.set_scan_optimization_mode(ScanMode::AllComponentsTogether);
        comp.set_quality(self.config.quality);

        comp.set_size(self.target_width, self.target_height);

        comp.set_mem_dest();
        comp.start_compress();

        let mut line = 0;
        loop {
            if line > self.target_height - 1 {
                break;
            }
            //&resized_img_data.
            comp.write_scanlines(
                &self.buf[line * self.target_width * 3..(line + 1) * self.target_width * 3],
            );
            line += 1;
        }
        comp.finish_compress();

        self.buf = comp
            .data_to_vec()
            .map_err(|_| anyhow!("data_to_vec failed"))?;
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let outfile = self.target_dir.join(&self.file_name);

        let mut file = BufWriter::new(fs::File::create(outfile)?);
        file.write_all(&self.buf)?;

        println!("{} is compressed!", &self.file_name);
        Ok(())
    }
}