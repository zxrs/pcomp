use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub quality: f32,
    pub resize: Resize,
    pub sharpness: Sharpness,
    pub brightness: i8,
    pub contrast: f32,
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
