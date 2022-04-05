#![allow(dead_code)]

use std::io::Write;
use core_dev::datetime::time_attributes::TimeAttributes;
use core_dev::datetime::datetime::get_current_datetime;
use core_dev::datetime::datetime::get_current_time;
use core_dev::datetime::time::minutes_to_time_struct;
use core_dev::datetime::time::sleep_by_secs;


use clap::Parser;


#[derive(Parser, Debug)]
#[clap(
    author = "Author: alexzanderr",
    version = include_str!("../../version"),
    about = "Rule 20 20 20 -> eye utility software to make you look away from screen for 20 seconds, every X minutes.",
    long_about = include_str!("../../ABOUT.md")
)]
struct Args {
    /// number of minutes as interval for 20 20 20 rule desktop notification
    #[clap(short = 'm', long = "minutes")]
    minutes: Option<u8>,

    #[clap(short = 's', long = "seconds")]
    seconds: Option<u8>,

    #[clap(long = "message")]
    message: Option<String>,
}

use ansi_term::Color::{
    Red,
    Yellow,
};

use soloud::*;

use std::collections::HashMap;


pub struct MusicPlayer<'a> {
    player:  Soloud,
    playing: bool,
    songs:   HashMap<&'a str, AudioFile<'a>>,
    handle:  Option<Handle>,
}

impl<'a> MusicPlayer<'a> {
    pub fn new() -> Result<Self, SoloudError> {
        let mut sl = Soloud::default()?;
        sl.set_global_volume(2.0);

        Ok(Self {
            player:  sl,
            playing: false,
            songs:   HashMap::new(),
            handle:  None,
        })
    }

    pub fn load_file(
        &mut self,
        alias: &'a str,
        path: &'a str,
    ) -> Result<(), SoloudError> {
        let wav = self.songs.entry(alias).or_insert(AudioFile::new(
            alias,
            path,
            Wav::default(),
        ));
        wav.wav.load(path)?;
        Ok(())
    }

    pub fn load_bytes(
        &mut self,
        alias: &'a str,
        path: &'a str,
        bytes: &[u8],
    ) -> Result<(), SoloudError> {
        let wav = self.songs.entry(alias).or_insert(AudioFile::new(
            alias,
            path,
            Wav::default(),
        ));
        wav.wav.load_mem(bytes)?;
        Ok(())
    }

    pub fn pause_playing(&mut self) {
        if let Some(handle) = self.handle {
            self.playing = false;
            self.player.set_pause(handle, true);
        }
    }

    pub fn continue_playing(&mut self) {
        if let Some(handle) = self.handle {
            self.playing = true;
            self.player.set_pause(handle, false);
        }
    }

    pub fn is_done_playing(&self) -> bool {
        self.player.voice_count() == 0
    }

    pub fn is_playing(&self) -> bool {
        self.player.voice_count() > 0
    }

    pub fn play_music(&mut self, alias: String) {
        self.playing = true;
        let wav = self.songs.get_mut(alias.as_str()).unwrap();
        let handle = self.player.play(&wav.wav);
        self.handle = Some(handle);
        while self.is_playing() {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    // ERROR
    // cannot borrow mutable and immutable at the same time
    // pub fn play_all(&mut self) {
    //     for (alias, audio_file) in self.songs.iter {
    //         self.play_music(alias);
    //     }
    // }
}

struct AudioFile<'a> {
    alias: &'a str,
    path:  &'a str,
    wav:   Wav,
}

impl<'a> AudioFile<'a> {
    pub fn new(alias: &'a str, path: &'a str, wav: Wav) -> Self {
        Self {
            alias,
            path,
            wav,
        }
    }
}


fn yellow_bold(string: &str) -> String {
    Yellow.bold().paint(string).to_string()
}


fn red_bold(string: &str) -> String {
    Red.bold().paint(string).to_string()
}


use color_backtrace::install as _install_colored_backtrace;
use core_dev::imagelib::ImageBufferType;

fn desktop_notification(message: &str, image_buffer: &ImageBufferType) {
    linux_notification_from_image_buffer(
        "RULE 20 20 20",
        message,
        image_buffer,
        10,
    )
}


// DEPRECATED
// fn playsound(path: &str) -> bool {
//     let command = format!(
//         "cvlc --play-and-exit --no-repeat {path} > /dev/null 2>&1"
//     );
//     let mut p = std::process::Command::new("sh")
//         .arg("-c")
//         .arg(command)
//         .spawn()
//         .expect("failed to spawm command");
//     let status = p.wait().unwrap();
//     status.success()
// }

fn comma_sound(sounds_map: &mut MusicPlayer) {
    sounds_map.play_music("comma".to_string());
}

fn warning_sound(sounds_map: &mut MusicPlayer) {
    sounds_map.play_music("warning".to_string());
}

fn bizwarn_sound(sounds_map: &mut MusicPlayer) {
    sounds_map.play_music("bizwarn".to_string());
}

fn number_sound<'a>(sounds_map: &mut MusicPlayer<'a>, number: String) {
    sounds_map.play_music(number);
}

fn order_sound(sounds_map: &mut MusicPlayer) {
    sounds_map.play_music("order".to_string());
}

fn rule_202020_sound(sounds_map: &mut MusicPlayer) {
    number_sound(sounds_map, "20".to_owned());
    number_sound(sounds_map, "20".to_owned());
    number_sound(sounds_map, "20".to_owned());
    order_sound(sounds_map);
}

fn countdown<'a>(
    sounds_map: &mut MusicPlayer<'a>,
    start: usize,
    pause: f32,
) {
    if start > 20 {
        panic!("start should be less than 20, cuz 20 20 20 rule boy.")
    }
    for i in (0..21).rev().into_iter() {
        let i_str = i.to_string();
        if i > 9 {
            println!("[  {}  ]", red_bold(&i_str));
        } else {
            println!("[   {}  ]", red_bold(&i_str));
        }
        number_sound(sounds_map, i.to_string());
        sleep_by_secs(pause);
    }
}

static BIZWARN_SOUND: &[u8; 9839] =
    include_bytes!("../../static/audio/bizwarn_minus_15.wav");
static WARNING_SOUND: &[u8; 6267] =
    include_bytes!("../../static/audio/warning_minus_15.wav");
static COMMA_SOUND: &[u8; 2788] =
    include_bytes!("../../static/audio/comma_minus_15.wav");
static ORDER_SOUND: &[u8; 6200] =
    include_bytes!("../../static/audio/order_minus_15.wav");
static ZERO_SOUND: &[u8; 7197] =
    include_bytes!("../../static/audio/numbers/zero_minus_15.wav");
static ONE_SOUND: &[u8; 5269] =
    include_bytes!("../../static/audio/numbers/one_minus_15.wav");
static TWO_SOUND: &[u8; 5639] =
    include_bytes!("../../static/audio/numbers/two_minus_15.wav");
static THREE_SOUND: &[u8; 5160] =
    include_bytes!("../../static/audio/numbers/three_minus_15.wav");
static FOUR_SOUND: &[u8; 5247] =
    include_bytes!("../../static/audio/numbers/four_minus_15.wav");
static FIVE_SOUND: &[u8; 7692] =
    include_bytes!("../../static/audio/numbers/five_minus_15.wav");
static SIX_SOUND: &[u8; 6098] =
    include_bytes!("../../static/audio/numbers/six_minus_15.wav");
static SEVEN_SOUND: &[u8; 6701] =
    include_bytes!("../../static/audio/numbers/seven_minus_15.wav");
static EIGHT_SOUND: &[u8; 5353] =
    include_bytes!("../../static/audio/numbers/eight_minus_15.wav");
static NINE_SOUND: &[u8; 6632] =
    include_bytes!("../../static/audio/numbers/nine_minus_15.wav");
static TEN_SOUND: &[u8; 5599] =
    include_bytes!("../../static/audio/numbers/ten_minus_15.wav");
static ELEVEN_SOUND: &[u8; 7140] =
    include_bytes!("../../static/audio/numbers/eleven_minus_15.wav");
static TWELVE_SOUND: &[u8; 5834] =
    include_bytes!("../../static/audio/numbers/twelve_minus_15.wav");
static THIRTEEN_SOUND: &[u8; 8487] =
    include_bytes!("../../static/audio/numbers/thirteen_minus_15.wav");
static FOURTEEN_SOUND: &[u8; 8864] =
    include_bytes!("../../static/audio/numbers/fourteen_minus_15.wav");
static FIFTEEN_SOUND: &[u8; 8936] =
    include_bytes!("../../static/audio/numbers/fifteen_minus_15.wav");
static SIXTEEN_SOUND: &[u8; 9356] =
    include_bytes!("../../static/audio/numbers/sixteen_minus_15.wav");
static SEVENTEEN_SOUND: &[u8; 11078] =
    include_bytes!("../../static/audio/numbers/seventeen_minus_15.wav");
static EIGHTTEEN_SOUND: &[u8; 7163] =
    include_bytes!("../../static/audio/numbers/eighteen_minus_15.wav");
static NINETEEN_SOUND: &[u8; 8673] =
    include_bytes!("../../static/audio/numbers/nineteen_minus_15.wav");
static TWENTY_SOUND: &[u8; 6497] =
    include_bytes!("../../static/audio/numbers/twenty_minus_15.wav");

/// you dont need to specify total bytes, you can mention just a slice ...
static NOTIFICATION_ICON: &[u8] =
    include_bytes!("../../static/icons/rule-202020.png");


use core_dev::core::set_keyboard_interrupt_handler;
use core_dev::imagelib::create_image_buffer_from_bytes;
use core_dev::linuxapi::linux_notification_from_image_buffer;

fn main() -> Result<(), SoloudError> {
    _install_colored_backtrace();

    let rule_202020_icon_image_buffer =
        create_image_buffer_from_bytes(NOTIFICATION_ICON);

    set_keyboard_interrupt_handler(move || {
        println!();
        println!("received SIGKILL");
        println!("application stopped");
        println!("exited with code 1");
        std::process::exit(1);
    });

    let mut sounds_map: HashMap<&str, &[u8]> = HashMap::new();
    sounds_map.insert(
        "bizwarn",
        include_bytes!("../../static/audio/bizwarn_minus_15.wav"),
    );
    sounds_map.insert(
        "warning",
        include_bytes!("../../static/audio/warning_minus_15.wav"),
    );
    sounds_map.insert("comma", COMMA_SOUND);
    sounds_map.insert("order", ORDER_SOUND);
    sounds_map.insert("0", ZERO_SOUND);
    sounds_map.insert("1", ONE_SOUND);
    sounds_map.insert("2", TWO_SOUND);
    sounds_map.insert("3", THREE_SOUND);
    sounds_map.insert("4", FOUR_SOUND);
    sounds_map.insert("5", FIVE_SOUND);
    sounds_map.insert("6", SIX_SOUND);
    sounds_map.insert("7", SEVEN_SOUND);
    sounds_map.insert("8", EIGHT_SOUND);
    sounds_map.insert("9", NINE_SOUND);
    sounds_map.insert("10", TEN_SOUND);
    sounds_map.insert("11", ELEVEN_SOUND);
    sounds_map.insert("12", TWELVE_SOUND);
    sounds_map.insert("13", THIRTEEN_SOUND);
    sounds_map.insert("14", FOURTEEN_SOUND);
    sounds_map.insert("15", FIFTEEN_SOUND);
    sounds_map.insert("16", SIXTEEN_SOUND);
    sounds_map.insert("17", SEVENTEEN_SOUND);
    sounds_map.insert("18", EIGHTTEEN_SOUND);
    sounds_map.insert("19", NINETEEN_SOUND);
    sounds_map.insert("20", TWENTY_SOUND);


    let mut music_player = MusicPlayer::new()?;
    for (alias, bytes) in sounds_map.iter() {
        music_player.load_bytes(alias, "asd", bytes)?;
    }


    let args = Args::parse();
    println!("{:?}", args);
    println!("{:?}", args.minutes);
    println!("{:?}", args.seconds);
    let seconds = if let Some(seconds) = args.seconds {
        seconds as usize
    } else {
        0
    };

    let minutes = if let Some(minutes) = args.minutes {
        minutes as usize
    } else {
        0
    };

    let mut default_message = String::from("ITS TIME NOW!!!");
    if let Some(message) = args.message {
        default_message = message;
    }

    let mut time_struct = minutes_to_time_struct(minutes);
    time_struct.seconds += seconds;
    time_struct.normalize();


    loop {
        let mut ta = time_struct.clone();
        println!("{:?}", ta);
        rule_202020(&mut ta, &mut music_player, &default_message, &rule_202020_icon_image_buffer);
    }
}


fn rule_202020(
    ta: &mut TimeAttributes,
    music_player: &mut MusicPlayer,
    message: &str,
    image_buffer: &ImageBufferType
) {
    let clock = yellow_bold(&ta.format_as_clock_with_level(2));
    print!("Time Left: [  {}  ]\r", clock);
    std::io::stdout().flush().expect("failed to flush stdout");
    std::thread::sleep(std::time::Duration::from_millis(1000));

    while !ta.all_zeros() {
        ta.decrement_seconds(1);
        let clock = yellow_bold(&ta.format_as_clock_with_level(2));
        print!("\x1b[2KTime Left: [  {}  ]\r", clock);
        ta.normalize_decrement();
        std::io::stdout().flush().expect("failed to flush stdout");

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    println!(
        "\n\n{} !!! ( {} )",
        red_bold("20 20 20 ORDER"),
        get_current_time()
    );

    for _ in 0..3 {
        warning_sound(music_player);
        comma_sound(music_player);
    }
    bizwarn_sound(music_player);
    rule_202020_sound(music_player);
    desktop_notification(message, &image_buffer);
    println!("notification sent at {}", get_current_datetime());
    countdown(music_player, 20, 1.0);
    println!()
}
