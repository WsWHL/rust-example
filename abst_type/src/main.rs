mod media;
mod car;

use crate::media::Playable;
use crate::car::{Car, TeslaRoadster, Vehicle};

struct Audio(String);
struct Video(String);

impl Playable for Audio {
    fn play(&self) {
        println!("Now playing: {}", self.0);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Now playing: {}", self.0);
    }
}

fn main() {
    println!("Super player!");

    let audio = Audio("ambient_music.mp3".to_string());
    let video = Video("big_buck_bunny.mkv".to_string());

    audio.play();
    video.play();

    // tesla car
    let roadster = TeslaRoadster::new("Tesla Roadster II", 2022);
    println!("model: {}", roadster.model());
    println!("{} is priced at ${}", roadster.model, roadster.get_price());
    assert_eq!(roadster.get_price(), 270000);
}
