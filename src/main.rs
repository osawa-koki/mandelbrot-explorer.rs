extern crate image;

use dotenv::dotenv;
use image::png::PNGEncoder;
use image::ColorType;
use ops::Pow;
use rug::*;
use std::env;
use std::fs;
use std::fs::File;

static RUG_PREC: u32 = 32 * 8;

fn escape_time(c: Complex, limit: u32) -> Option<u32> {
    let mut z = Complex::with_val(RUG_PREC, (0.0, 0.0));
    for i in 0..limit {
        z = z.clone() * z.clone() + c.clone();
        if &(4.0) < z.clone().norm().real() {
            return Some(i);
        }
    }
    None
}

fn pixel2point_cenverter(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex,
    lower_right: Complex,
) -> Complex {
    let (width, height) = (
        Float::with_val(RUG_PREC, lower_right.real() - upper_left.real()),
        Float::with_val(RUG_PREC, upper_left.imag() - lower_right.imag()),
    );
    Complex::with_val(
        RUG_PREC,
        (
            upper_left.real() + pixel.0 as f64 * width / bounds.0 as f64,
            upper_left.imag() - pixel.1 as f64 * height / bounds.1 as f64,
        ),
    )
}

static THRESHOULD: u32 = 255;

fn render(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex, lower_right: Complex) {
    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point =
                pixel2point_cenverter(bounds, (col, row), upper_left.clone(), lower_right.clone());
            pixels[row * bounds.0 + col] = match escape_time(point, THRESHOULD) {
                None => 0,
                Some(count) => (THRESHOULD - count as u32).try_into().unwrap(),
            };
        }
    }
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let file = File::create(filename)?;
    let encoder = PNGEncoder::new(file);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}

fn handler(path: &str, bounds: (usize, usize), upper_left: Complex, lower_right: Complex) {
    let mut pixels = vec![0; bounds.0 * bounds.1];
    render(&mut pixels, bounds, upper_left, lower_right);
    write_image(path, &pixels, bounds).expect("error while writing PNG file.");
}

fn mkdir(dir_name: &str) {
    match fs::create_dir(dir_name) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }
}

fn main() {
    dotenv().ok();

    // 環境変数を文字列として取得。
    let from_env = env::var("FROM").expect("uncaught env var \"START\"");
    let upto_env = env::var("UPTO").expect("uncaught env var \"UPTO\"");
    let aspect_ratio_env = env::var("ASPECT_RATIO").expect("uncaught env var \"ASPECT_RATIO\"");
    let shrink_ratio_env = env::var("SHRINK_RATIO").expect("uncaught env var \"SHRINK_RATIO\"");
    let file_size_height_env =
        env::var("FILESIZE_HEIGHT").expect("uncaught env var \"FILESIZE_HEIGHT\"");
    let start_x_env = env::var("START_X").expect("uncaught env var \"START_X\"");
    let start_y_env = env::var("START_Y").expect("uncaught env var \"START_Y\"");
    let default_width_env = env::var("DEFAULT_WIDTH").expect("uncaught env var \"DEFAULT_WIDTH\"");
    let default_height_env =
        env::var("DEFAULT_HEIGHT").expect("uncaught env var \"DEFAULT_HEIGHT\"");

    // 環境変数を型変換。
    let start: usize = from_env.parse().expect("\"START\" param must be INT type");
    let upto: usize = upto_env.parse().expect("\"UPTO\" param must be INT type");
    let aspect_ratio: f64 = aspect_ratio_env
        .parse()
        .expect("\"ASPECT_RATIO\" param must be FLOAT type");
    let shrink_ratio: f64 = shrink_ratio_env
        .parse()
        .expect("\"SHRINK_RATIO\" param must be FLOAT type");
    let file_size_height: usize = file_size_height_env
        .parse()
        .expect("\"FILESIZE_HEIGHT\" param must be INT type");
    let start_x: Float = Float::with_val(
        RUG_PREC,
        Float::parse(start_x_env).expect("\"START_X\" param must be FLOAT type"),
    );
    let start_y: Float = Float::with_val(
        RUG_PREC,
        Float::parse(start_y_env).expect("\"START_Y\" param must be FLOAT type"),
    );
    let default_width: f64 = default_width_env
        .parse()
        .expect("\"DEFAULT_WIDTH\" param must be FLOAT type");
    let default_height: f64 = default_height_env
        .parse()
        .expect("\"DEFAULT_HEIGHT\" param must be FLOAT type");

    mkdir(&format!("./dist/"));

    for i in start..upto {
        println!(
            "{}% | {} / {}",
            (i - start) * 100 / (upto - start),
            i - start,
            upto - start
        );

        let height = Float::with_val(RUG_PREC, default_height)
            * Float::with_val(RUG_PREC, shrink_ratio).pow(i as u32);
        let c_size_x =
            Float::with_val(RUG_PREC, height.clone()) * Float::with_val(RUG_PREC, aspect_ratio);
        let c_size_y = Float::with_val(RUG_PREC, height.clone());
        let new_start_x: Float = start_x.clone() + ((default_width - c_size_x.clone()) / 2.0);
        let new_start_y: Float = start_y.clone() - ((default_height - c_size_y.clone()) / 2.0);
        let upper_left = Complex::with_val(RUG_PREC, (new_start_x.clone(), new_start_y.clone()));
        let lower_right = Complex::with_val(
            RUG_PREC,
            (
                new_start_x.clone() + c_size_x.clone(),
                new_start_y.clone() - c_size_y.clone(),
            ),
        );
        handler(
            &format!("./dist/{0: >08}.png", i),
            (
                (file_size_height as f64 * aspect_ratio) as usize,
                file_size_height as usize,
            ),
            upper_left,
            lower_right,
        )
    }
}
