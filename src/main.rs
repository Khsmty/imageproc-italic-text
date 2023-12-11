use image::{imageops::overlay, DynamicImage, Rgba, RgbaImage};
use imageproc::geometric_transformations::{warp_into, Interpolation, Projection};
use imageutil::text::{empty_resolver, Fonts};
use once_cell::sync::Lazy;
use rusttype::{Font, Scale};

const INTER: Lazy<Font> =
    Lazy::new(|| Font::try_from_bytes(include_bytes!("../fonts/Inter-Regular.ttf")).unwrap());
const FONT_SETTING: Lazy<Vec<Font<'static>>> = Lazy::new(|| vec![INTER.clone()]);

#[tokio::main]
async fn main() {
    let mut img = DynamicImage::new_rgba8(400, 100);
    let fonts = Fonts::new(FONT_SETTING.clone());

    let mut text_image = DynamicImage::new_rgba8(300, 80);
    let mut italic_text_image = RgbaImage::new(300, 80);
    fonts
        .write_to(
            &mut text_image,
            "Hello, World!",
            Scale::uniform(60.0),
            Rgba::from([0, 0, 0, 255]),
            0,
            0,
            empty_resolver,
        )
        .await;

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
