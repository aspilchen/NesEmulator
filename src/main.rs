mod nes;
use nes::{cart::Cart, open_ines_file, Nes};
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};

use crate::nes::ppu::show_tile_bank;

fn show_tiles(cart: Cart) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Tile viewer", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    let right_bank = show_tile_bank(&cart.chr_rom, 1);

    texture.update(None, &right_bank.data, 256 * 3).unwrap();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                _ => { /* do nothing */ }
            }
        }
    }
}

fn main() {
    let cart = open_ines_file("super_mario.nes");
    // println!("{:?}", cart.header);
    // println!("{:X}", cart.header.control.get_mapper());
    show_tiles(cart);
}
