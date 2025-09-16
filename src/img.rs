use image::{DynamicImage, Pixel, Rgb, Rgba};
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::Pref;

const BASE_IMG_PATH: &str = "./data/maps/full.png";

pub struct PrefImgGenerator {
    img: DynamicImage,
    size: u32,
}

impl PrefImgGenerator {
    pub fn new(size: u32) -> Self {
        let img = image::open(BASE_IMG_PATH)
            .expect("failed to open img.")
            .resize(size, size, image::imageops::FilterType::CatmullRom);

        Self { img, size }
    }

    pub fn overlay(&mut self, pref: &Pref, tint_color: &Rgb<u8>) {
        let pref_img = image::open(format!("./data/maps/{}.png", pref.as_key()))
            .expect("failed to fetch pref img")
            .resize(
                self.size,
                self.size,
                image::imageops::FilterType::CatmullRom,
            );

        let pref_img = pref_img
            .as_rgba8()
            .expect("failed to parse pref image to rgb8");

        let base_img = self
            .img
            .as_mut_rgba8()
            .expect("failed to parse color type as rgba8");

        base_img
            .pixels_mut()
            .zip(pref_img.pixels())
            .par_bridge()
            .for_each(|(base, pref)| {
                let tinted_pixel = tint_pixel(pref, tint_color);
                base.blend(&tinted_pixel);
            });

        let img = DynamicImage::ImageRgba8(base_img.to_owned());

        self.img = img;
    }

    pub fn get_img(&self) -> DynamicImage {
        self.img.clone()
    }
}

fn tint_pixel(pixel: &Rgba<u8>, tint_color: &Rgb<u8>) -> Rgba<u8> {
    let channels = pixel
        .channels()
        .iter()
        .take(3)
        .zip(tint_color.channels())
        .map(|(&p, &t)| (p as f64 * (t as f64 / 255.0)).clamp(0.0, 255.0) as u8)
        .collect::<Vec<_>>();

    let [r, g, b] = channels.as_slice() else {
        panic!("failed to parse rgb");
    };
    let a = pixel.0[3];

    Rgba::from([*r, *g, *b, a])
}
