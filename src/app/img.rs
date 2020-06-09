use super::Config;
use anyhow::{anyhow, bail, Result};
use image::{self, imageops::FilterType, DynamicImage, GenericImageView, RgbImage};
use mozjpeg::{ColorSpace, Compress, Decompress, ScanMode};
use std::fs;
use std::io::{prelude::*, BufWriter};
use std::path::Path;

pub struct Img<'a> {
    img: DynamicImage,
    config: &'a Config,
    path: &'a Path,
    buf: Vec<u8>,
    origin_size: usize,
}

impl<'a> Img<'a> {
    pub fn open(config: &'a Config, path: &'a Path) -> Result<Self> {
        let (data, width, height, origin_size) = {
            let comp_data = fs::read(path)?;
            let origin_size = comp_data.len();
            let mut d = Decompress::new_mem(&comp_data)?.rgb()?;
            let width = d.width() as u32;
            let height = d.height() as u32;
            let data = d
                .read_scanlines::<[u8; 3]>()
                .ok_or(anyhow!("read_scanlines is none"))?
                .iter()
                .flatten()
                .cloned()
                .collect::<Vec<_>>();
            d.finish_decompress();
            (data, width, height, origin_size)
        };

        let image_buffer = RgbImage::from_raw(width, height, data)
            .ok_or(anyhow!("RgbImage::from_raw is none"))?;
        let img = DynamicImage::ImageRgb8(image_buffer);

        Ok(Img {
            img,
            config,
            path,
            buf: vec![],
            origin_size,
        })
    }

    pub fn contrast(&mut self) {
        if self.config.contrast.enable {
            self.img = self.img.adjust_contrast(self.config.contrast.setting);
        }
    }

    pub fn brighten(&mut self) {
        if self.config.brightness.enable {
            self.img = self.img.brighten(self.config.brightness.setting as i32);
        }
    }

    pub fn sharpen(&mut self) {
        if self.config.sharpness.enable {
            self.img = self
                .img
                .unsharpen(self.config.sharpness.sigma, self.config.sharpness.threshold);
        }
    }

    pub fn resize(&mut self) {
        if self.config.resize.enable {
            self.img = self.img.resize(
                self.config.resize.long_side_length as u32,
                self.config.resize.long_side_length as u32,
                FilterType::Lanczos3,
            );
        }
    }

    pub fn compress(&mut self) -> Result<()> {
        let width = self.img.width() as usize;
        let height = self.img.height() as usize;
        let buf = self.img.to_rgb().to_vec();

        let mut comp = Compress::new(ColorSpace::JCS_RGB);
        comp.set_scan_optimization_mode(ScanMode::AllComponentsTogether);
        comp.set_quality(self.config.general.jpeg_quality);
        comp.set_size(width, height);
        comp.set_mem_dest();
        comp.start_compress();

        let mut line = 0;
        loop {
            if line > height - 1 {
                break;
            }
            comp.write_scanlines(&buf[line * width * 3..(line + 1) * width * 3]);
            line += 1;
        }
        comp.finish_compress();

        self.buf = comp
            .data_to_vec()
            .map_err(|_| anyhow!("data_to_vec failed"))?;
        Ok(())
    }

    pub fn save(&self) -> Result<u8> {
        let outfile = if self.config.general.overwrite_existing_files {
            self.path.to_owned()
        } else {
            let file_name = self.path.file_name().ok_or(anyhow!("no file name"))?;
            self.path.join("../compressed").join(file_name)
        };
        let mut file = BufWriter::new(fs::File::create(outfile)?);
        file.write_all(&self.buf)?;
        
        let ratio = (self.origin_size as f32 / self.buf.len() as f32 * 100.0) as u8;
        Ok(ratio)
    }
}
