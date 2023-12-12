use image::{imageops::overlay, DynamicImage, Rgba, RgbaImage};
use imageproc::{
    drawing::draw_text_mut,
    geometric_transformations::{warp_into, Interpolation, Projection},
};
use once_cell::sync::Lazy;
use rusttype::{Font, Scale};

const INTER: Lazy<Font> =
    Lazy::new(|| Font::try_from_bytes(include_bytes!("../fonts/Inter-Regular.ttf")).unwrap());

#[tokio::main]
async fn main() {
    let mut img = DynamicImage::new_rgba8(400, 100);

    let mut text_image = DynamicImage::new_rgba8(300, 80);
    let mut italic_text_image = RgbaImage::new(300, 80);
    draw_text_mut(
        &mut text_image,
        Rgba::from([0, 0, 0, 255]),
        0,
        0,
        Scale::uniform(60.0),
        &INTER.clone(),
        "Hello, World!",
    );

    let matrix = Projection::from_matrix([
        1_f32,    // 固定
        -0.2_f32, // 傾きの強さ (値を小さくすればするほど強くなる)
        4.8_f32,  // 傾けると x 軸方向がズレて切れてしまうので、いい感じに修正する
        0_f32,    // 固定
        1_f32,    // 固定
        0_f32,    // 固定
        0_f32,    // 固定
        0_f32,    // 固定
        1_f32,    // 固定
    ]);
    warp_into(
        &text_image.to_rgba8(),
        &matrix.unwrap(),
        Interpolation::Bilinear,
        Rgba([0, 0, 0, 0]),
        &mut italic_text_image,
    );

    overlay(&mut img, &italic_text_image, 10, 10);

    img.save("image.png").unwrap()
}
