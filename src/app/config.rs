use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub quality: f32,
    pub resize: Resize,
    pub sharpness: Sharpness,
    pub brightness: Brightness,
    pub contrast: Contrast,
}

#[derive(Deserialize)]
pub struct Resize {
    pub enable: bool,
    pub long_side_length: usize,
}

#[derive(Deserialize)]
pub struct Sharpness {
    pub enable: bool,
    pub sigma: f32,
    pub threshold: i32,
}

#[derive(Deserialize)]
pub struct Brightness {
    pub enable: bool,
    pub setting: i8,
}

#[derive(Deserialize)]
pub struct Contrast {
    pub enable: bool,
    pub setting: f32,
}
