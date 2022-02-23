
use std::collections::HashMap;

pub const AUDIO_FILE_AWESOMENESS: &[u8] = include_bytes!("../assets/audio/awesomeness.wav");
pub const AUDIO_FILE_SHOOT: &[u8] = include_bytes!("../assets/audio/shoot.wav");
pub const AUDIO_FILE_SHOOTED: &[u8] = include_bytes!("../assets/audio/shooted.wav");

pub enum Sink {
    Regular(rodio::Sink),
    SpatialSink(rodio::SpatialSink),
}

impl Sink {
    pub fn set_volume(&mut self, volume: f32) {
        match self {
            Sink::Regular(s) => {
                s.set_volume(volume);
            }
            Sink::SpatialSink(s) => {
                s.set_volume(volume);
            }
        };
    }

    pub fn empty(&self) -> bool {
        match self {
            Sink::Regular(s) => s.empty(),
            Sink::SpatialSink(s) => s.empty(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct AudioContext {
    pub output_stream_handle: rodio::OutputStreamHandle,
    pub output_stream: rodio::OutputStream,
    pub global_sinks_map: HashMap<&'static str, Sink>,
    pub global_sinks_array: Vec<Sink>,
    pub volume: f32,
}
#[cfg(target_arch = "wasm32")]
pub struct AudioContext {
    pub global_sinks_map: HashMap<&'static str, Sink>,
    pub global_sinks_array: Vec<Sink>,
    pub volume: f32,
}

impl AudioContext {
    pub fn new() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let (output_stream, output_stream_handle) = rodio::OutputStream::try_default().unwrap();

        #[cfg(not(target_arch = "wasm32"))]
        let ret = Self {
            global_sinks_map: HashMap::new(),
            global_sinks_array: Vec::new(),
            volume: 1.0,
            output_stream,
            output_stream_handle,
        };
        #[cfg(target_arch = "wasm32")]
        let ret = Self {
            global_sinks_map: HashMap::new(),
            global_sinks_array: Vec::new(),
            volume: 1.0,
        };
        ret
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.volume = volume;
            for (_k, v) in self.global_sinks_map.iter_mut() {
                v.set_volume(volume);
            }
            for sink in self.global_sinks_array.iter_mut() {
                sink.set_volume(volume);
            }
        }
    }

    pub fn push(&mut self, mut sink: Sink) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            sink.set_volume(self.volume);
            self.global_sinks_array.push(sink);
        }
    }

    pub fn insert(&mut self, key: &'static str, mut sink: Sink) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            sink.set_volume(self.volume);
            self.global_sinks_map.insert(key, sink);
        }
    }

    pub fn clear(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.global_sinks_map.retain(|_k, v| !v.empty());
            self.global_sinks_array.retain(|v| !v.empty());
        }
    }
}
