use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub max_length: usize,
    pub quality: f32,
    pub sharpness: Sharpness,
    pub brightness: i8,
    pub contrast: f32,
}

#[derive(Deserialize)]
pub struct Sharpness {
    pub enable: bool,
    pub sigma: f32,
    pub threshold: i32,
}
