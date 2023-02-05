// use image::GenericImageView;
use image::{GenericImageView, RgbImage};
use show_image::{create_window, event, ImageInfo, ImageView};
use std::{thread, time};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open("image.jpg").expect("File not found!");
    let (width, height) = img.dimensions();
    println!("Resolution: {}x{}", width, height);
    let mut pixels: Vec<u8> = Vec::new();

    for pixel in img.pixels() {
        pixels.push(pixel.2 .0[0]);
        pixels.push(pixel.2 .0[1]);
        pixels.push(pixel.2 .0[2]);
        // pixels.push(pixel.2.0[3]);
    }

    // println!("{:?}", pixels);
    let image = ImageView::new(ImageInfo::rgb8(width, height), &pixels);
    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image)?;

    // let ten_millis = time::Duration::from_millis(5000);
    // thread::sleep(ten_millis);

    // Enter key が押されたら終了
    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Return)
                && event.input.state.is_pressed()
            {
                break;
            }
        }
    }

    Ok(())
}
