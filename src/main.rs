use std::{env, fs};

use event_handler::SdlEventsHandler;
use gamuboy::{
    config::Config,
    gameboy::GameBoy,
    lcd::{self},
    mode::Mode,
    saver::FileSaver,
};
use gui::Gui;
use sdl2::{event::Event, pixels::PixelFormatEnum, render::Texture};
use stereo::SdlStereoPlayer;

mod event_handler;
mod gui;
mod stereo;

const INITIAL_SCALE: f32 = 3.;

fn get_string_argument(argv: &Vec<String>, flag: &str) -> Option<String> {
    let index = argv
        .into_iter()
        .position(|x| x.as_str() == format!("--{}", flag));

    match index {
        Some(index) => {
            if index + 1 > argv.len() {
                return None;
            }
            Some(argv[index + 1].clone())
        }
        None => None,
    }
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() < 2 {
        panic!("Rom path is required!");
    }

    let rom_path = &argv[1];

    let bootrom_path = get_string_argument(&argv, "bootrom");

    let rom = match fs::read(rom_path) {
        Ok(rom) => rom,
        Err(err) => panic!("Error occured reading rom: {}", err),
    };

    let cfg = Config {
        mode: match rom[0x143] {
            0x80 | 0xC0 => Mode::CGB,
            _ => Mode::DMG,
        },
        rom,
        headless_mode: false,
        bootrom: match bootrom_path {
            Some(bootrom_path) => match fs::read(bootrom_path) {
                Ok(bootrom) => Some(bootrom),
                Err(err) => panic!("Error occured reading bootrom: {}", err),
            },
            None => None,
        },
        log_file_path: None,
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let window = video_subsystem
        .window(
            "Gamuboy",
            lcd::PIXELS_WIDTH as u32 * INITIAL_SCALE as u32,
            lcd::PIXELS_HEIGHT as u32 * INITIAL_SCALE as u32,
        )
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.set_scale(INITIAL_SCALE, INITIAL_SCALE).unwrap();

    let texture_creator = canvas.texture_creator();
    let texture: Texture<'static> = unsafe {
        std::mem::transmute(
            texture_creator
                .create_texture_streaming(
                    PixelFormatEnum::RGB24,
                    lcd::PIXELS_WIDTH as u32,
                    lcd::PIXELS_HEIGHT as u32,
                )
                .unwrap(),
        )
    };

    let (event_tx, event_rx) = std::sync::mpsc::channel::<Event>();

    let game_controller_subsystem = sdl_context.game_controller().unwrap();

    let available = game_controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))
        .unwrap();

    let controller = (0..available).find_map(|id| {
        if !game_controller_subsystem.is_game_controller(id) {
            println!("{} is not a game controller", id);
            return None;
        }

        println!("Attempting to open controller {}", id);

        match game_controller_subsystem.open(id) {
            Ok(c) => {
                println!("Success: opened \"{}\"", c.name());
                Some(c)
            }
            Err(e) => {
                println!("failed: {:?}", e);
                None
            }
        }
    });

    let mut gb = GameBoy::new(
        &cfg,
        Gui::new(canvas, texture),
        SdlStereoPlayer::new(&audio_subsystem),
        SdlEventsHandler::new(controller),
        FileSaver::new().unwrap(),
        &event_rx,
    );

    let mut event_pump = sdl_context.event_pump().unwrap();

    // TODO: manage this in a better way
    const EVENT_POLLING_PERIOD: i32 = 16000;
    for mut i in 0.. {
        i = (i + 1) % EVENT_POLLING_PERIOD;
        if i == 0 {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => return,
                    sdl2::event::Event::Window { .. } => {}
                    _ => event_tx.send(event).unwrap(),
                }
            }
        }

        gb.step();
    }
}
