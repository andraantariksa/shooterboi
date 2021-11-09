use ambisonic::rodio::Source;
use ambisonic::{rodio, Ambisonic, AmbisonicBuilder};
use std::convert::TryInto;
use std::io::{BufReader, Cursor};
use std::sync::{Arc, Mutex};

pub struct AudioContext {
    pub driver: Ambisonic,
    // pub resource: AudioResource,
}

impl AudioContext {
    pub fn new() -> Self {
        Self {
            driver: AmbisonicBuilder::default().build(),
            // resource: AudioResource::new(),
        }
    }

    pub fn play(&self) {
        self.driver.play_omni(
            rodio::Decoder::new(Cursor::new(include_bytes!("assets/shooted.wav").to_vec()))
                .unwrap()
                .convert_samples(),
        );
    }
}

pub struct AudioResource {
    resource_map: Vec<Box<dyn rodio::Source<Item = f32> + Send>>,
}

impl AudioResource {
    fn new() -> Self {
        Self {
            resource_map: vec![
                Box::new(
                    rodio::Decoder::new(Cursor::new(include_bytes!("assets/shoot.wav").to_vec()))
                        .unwrap()
                        .convert_samples(),
                ),
                Box::new(
                    rodio::Decoder::new(Cursor::new(include_bytes!("assets/shooted.wav").to_vec()))
                        .unwrap()
                        .convert_samples(),
                ),
            ],
        }
    }
}
