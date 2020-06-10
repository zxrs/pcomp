use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub general: General,
    pub resize: Resize,
    pub sharpen: Sharpen,
    pub brighten: Brighten,
    pub contrast: Contrast,
}

#[derive(Deserialize)]
pub struct General {
    pub jpeg_quality: f32,
    pub num_threads: usize,
    pub read_sub_dir: bool,
    pub overwrite_existing_files: bool,
    pub keep_original_exif: bool,
}

#[derive(Deserialize)]
pub struct Resize {
    pub enable: bool,
    pub long_side_length: usize,
}

#[derive(Deserialize)]
pub struct Sharpen {
    pub enable: bool,
    pub sigma: f32,
    pub threshold: i32,
}

#[derive(Deserialize)]
pub struct Brighten {
    pub enable: bool,
    pub setting: i8,
}

#[derive(Deserialize)]
pub struct Contrast {
    pub enable: bool,
    pub setting: f32,
}
