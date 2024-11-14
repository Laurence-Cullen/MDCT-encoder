use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;
use std::path::Path;
use std::sync::{Arc, Mutex};

const RUNTIME_SECONDS: f64 = 10.0;
const AMPLIFICATION: f32 = 100.0;

fn main() {
    let host = cpal::default_host();

    let mic = host
        .default_input_device()
        .expect("no input device available");

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let config: StreamConfig = mic.default_input_config().unwrap().into();

    let all_input = Arc::new(Mutex::new(Vec::with_capacity(
        (RUNTIME_SECONDS * config.sample_rate.0 as f64) as usize,
    )));

    let input_copy = all_input.clone();

    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let mut output_fell_behind = false;

        let amplified = data.iter().map(|x| x * AMPLIFICATION);

        for sample in amplified {
            input_copy.lock().unwrap().push(sample);
        }
        if output_fell_behind {
            eprintln!("output stream fell behind: try increasing latency");
        }
    };

    let input_stream = mic
        .build_input_stream(&config, input_data_fn, err_fn, None)
        .unwrap();

    input_stream.play().unwrap();

    let out_fp: &Path = &Path::new("wav.wav");

    std::thread::sleep(std::time::Duration::from_secs(RUNTIME_SECONDS as u64));
    drop(input_stream);

    wavers::write(
        out_fp,
        &all_input.lock().unwrap(),
        config.sample_rate.0 as i32,
        config.channels,
    )
    .unwrap();
}
