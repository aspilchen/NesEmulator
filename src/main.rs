mod nes;
use nes::{cart::Cart, open_ines_file, Nes};
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};

use crate::nes::ppu::{
    chr_rom::{ChrBank, ChrRom},
    Frame, SYSTEM_PALLETE,
};

pub fn render_tile_bank(bank: &ChrBank) -> Frame {
    let mut frame = Frame::new();
    for (i, tile) in bank.tiles.iter().enumerate() {
        for tile_row in 0..=7 {
            for tile_col in 0..=7 {
                let value = tile.read(tile_row, tile_col);
                let rgb = match value {
                    0 => SYSTEM_PALLETE[0x01],
                    1 => SYSTEM_PALLETE[0x23],
                    2 => SYSTEM_PALLETE[0x27],
                    3 => SYSTEM_PALLETE[0x30],
                    _ => panic!("can't be"),
                };
                let col = ((i * 8) + tile_col) % 256;
                let row = ((i / 32) * 8) + tile_row;
                println!("{} {}", col, row);
                frame.write(col, row, rgb)
            }
        }
    }
    frame
}

fn show_frame(frame: &Frame) {
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

    texture.update(None, &frame.data, 256 * 3).unwrap();
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
    let cart = open_ines_file("pacman.nes");
    let chr_rom = ChrRom::try_from(cart.chr_rom.as_slice()).expect("broken");
    let frame = render_tile_bank(&chr_rom.banks[0]);
    show_frame(&frame);
}
