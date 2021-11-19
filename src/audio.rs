use std::convert::TryInto;
use std::io::{BufReader, Cursor};
use std::sync::{Arc, Mutex};

pub const SINK_ID_MAIN_MENU_BGM: usize = 0;

pub struct AudioContext {
    pub output_stream_handle: rodio::OutputStreamHandle,
    pub output_stream: rodio::OutputStream,
    pub global_sinks: Vec<rodio::Sink>,
}

impl AudioContext {
    pub fn new() -> Self {
        let (output_stream, output_stream_handle) = rodio::OutputStream::try_default().unwrap();

        let mut bgm_sink = rodio::Sink::try_new(&output_stream_handle).unwrap();
        bgm_sink.append(
            rodio::Decoder::new(Cursor::new(
                include_bytes!("../assets/audio/little-town.ogg").to_vec(),
            ))
            .unwrap(),
        );

        Self {
            global_sinks: vec![bgm_sink],
            output_stream: output_stream,
            output_stream_handle: output_stream_handle,
        }
    }

    pub fn get_sink_mut(&mut self, id: usize) -> &mut rodio::Sink {
        &mut self.global_sinks[id]
    }
}
