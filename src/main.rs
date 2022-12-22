// use image::GenericImageView;
use image::{GenericImageView, RgbImage};
use show_image::{event, ImageView, ImageInfo, create_window};
use std::{thread, time};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let img = image::open("image.jpg").expect("File not found!");
    let mut pixels: Vec<u8> = Vec::new();

    for pixel in img.pixels() {
        pixels.push(pixel.2.0[0]);
        pixels.push(pixel.2.0[1]);
        pixels.push(pixel.2.0[2]);
        // pixels.push(pixel.2.0[3]);
    }

    // println!("{:?}", pixels);
    let image = ImageView::new(ImageInfo::rgb8(640, 360), &pixels);
    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image)?;

    let ten_millis = time::Duration::from_millis(5000);
    thread::sleep(ten_millis);

    // Enter key が押されたら終了
    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Return) && event.input.state.is_pressed() {
                break;
            }
        }
    }

    Ok(())
}

/*
fn main() {
    let img = image::open("image.jpg").unwrap();
    println!("hello world");
    println!("dimenstions {:?}", img.dimensions());

    let mut img = image::RgbImage::new(512, 512);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        println!("x: {}", x);
        println!("y: {}",y);
        println!("pixel: {:?}", pixel);
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }
    img.save("result.png").unwrap();
}

 */




// fn main() {
//     println!("Hello, world!");
// }
