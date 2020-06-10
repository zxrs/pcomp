use anyhow::{bail, Result};
use rayon::prelude::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod img;
use img::Img;

mod config;
use config::Config;

pub struct App {
    files: Vec<PathBuf>,
    config: Config,
}

impl App {
    pub fn new() -> Result<Self> {
        let config: Config = toml::from_str(&fs::read_to_string("pcomp.toml")?)?;
        let files = env::args()
            .skip(1)
            .filter_map(|arg| read_arg(arg, &config))
            .flatten()
            .collect::<Vec<PathBuf>>();
        Ok(App { files, config })
    }

    pub fn start(&self) -> Result<()> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.config.general.num_threads)
            .build()?;

        pool.install(|| {
            self.files.iter().for_each(|path| {
                if let Some(file_name) = path.file_name() {
                    let file_name = file_name.to_string_lossy();
                    match self.process(path) {
                        Ok(ratio) => println!(
                            "{:>13} has been compressed to {:>2}% of its original size.",
                            file_name, ratio
                        ),
                        Err(e) => println!("{:>13} fails to compress due to \"{}\".", file_name, e),
                    }
                }
            });
        });
        Ok(())
    }

    fn process(&self, path: &Path) -> Result<u8> {
        if let Some(ext) = path.extension() {
            if !ext.to_string_lossy().as_ref().to_lowercase().eq("jpg")
                && !ext.to_string_lossy().as_ref().to_lowercase().eq("jpeg")
            {
                bail!("not jpeg file");
            }
        }

        let mut img = Img::open(&self.config, path)?;
        img.resize();
        img.contrast();
        img.brighten();
        img.sharpen();
        img.compress()?;
        let ratio = img.save()?;
        Ok(ratio)
        //Ok(0)
    }
}

fn read_arg(arg: String, config: &Config) -> Option<Vec<PathBuf>> {
    let path = PathBuf::from(arg);
    let mut v = vec![];
    if path.is_dir() {
        for entry in fs::read_dir(&path).ok()? {
            let entry = entry.ok()?;
            let file_type = entry.file_type().ok()?;
            if file_type.is_dir() && config.general.read_sub_dir {
                read_sub_dir(&entry.path(), &mut v).ok()?;
            } else if file_type.is_file() {
                v.push(entry.path());
            }
        }
        Some(v)
    } else if path.is_file() {
        v.push(path);
        Some(v)
    } else {
        None
    }
}

fn read_sub_dir(path: &Path, v: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            read_sub_dir(&entry.path(), v)?;
        } else if file_type.is_file() {
            v.push(entry.path());
        }
    }
    Ok(())
}
