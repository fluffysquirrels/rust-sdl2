extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use ::sdl2::messagebox::*;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_draw_color(Color::RGB(255, 0, 0));
    renderer.clear();
    renderer.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    let res =
                        show_simple_message_box(MESSAGEBOX_ERROR,
                                                "Some title",
                                                "Some information inside the window",
                                                renderer.window());
                    match res {
                        Ok(_) => {}
                        Err(ShowMessageError::SdlError(string)) => {
                            println!("An error occured : {}",string);
                        }
                        Err(_) => println!("Unexpected error occured !"),
                    };
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }
    let buttons : Vec<_> = vec![
        ButtonData {
            flags:MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT,
            button_id:1,
            text:"Ok"
        },
        ButtonData {
            flags:MESSAGEBOX_BUTTON_NOTHING,
            button_id:2,
            text:"No"
        },
        ButtonData {
            flags:MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT,
            button_id:3,
            text:"Cancel"
        },
    ];
    let res = show_message_box(MESSAGEBOX_WARNING,
                               buttons.as_slice(),
                               "Some warning",
                               "You forget to do something, do it anyway ?",
                               None,
                               None);
    println!("{:?}",res);
}
