use anyhow::{anyhow, bail, Result};
use rayon::prelude::*;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod img;
use img::Img;

#[derive(Deserialize)]
pub struct Config {
    max_length: usize,
    quality: f32,
    sharpness: i16
}

pub struct App {
    target_dir: PathBuf,
    files: Vec<PathBuf>,
    config: Config,
}

impl App {
    pub fn new() -> Result<Self> {
        let source = env::args()
            .nth(1)
            .map(|s| PathBuf::from(s))
            .ok_or(anyhow!("No source"))?;

        let (target_dir, files) = if source.is_dir() {
            let target_dir = source.join("compressed");
            let files: Vec<PathBuf> = fs::read_dir(&source)?
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    if entry.file_type().ok()?.is_file() {
                        Some(entry.path())
                    } else {
                        None
                    }
                })
                .collect();
            (target_dir, files)
        } else if source.is_file() {
            let target_dir = source
                .parent()
                .ok_or(anyhow!("No source parent dir."))?
                .join("compressed");
            let files: Vec<PathBuf> = env::args()
                .skip(1)
                .map(PathBuf::from)
                .filter(|p| p.is_file())
                .collect();
            (target_dir, files)
        } else {
            bail!("Invalid source.")
        };

        let config = toml::from_str(&fs::read_to_string("pcomp.toml")?)?;

        Ok(App {
            target_dir,
            files,
            config,
        })
    }

    pub fn start(&self) -> Result<()> {
        if !self.target_dir.exists() {
            fs::create_dir(&self.target_dir)?;
        }

        self.files.par_iter().for_each(|path| {
            if let Err(e) = self.process(path) {
                println!("{}", e);
            }
        });
        Ok(())
    }

    fn process(&self, path: &Path) -> Result<()> {
        let mut img = Img::open(&self.config, path, &self.target_dir)?;
        //img.brighten()?;
        img.sharpen()?;
        img.resize()?;
        img.compress()?;
        img.save()?;
        Ok(())
    }
}
