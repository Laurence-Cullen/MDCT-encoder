mod mdct;
mod record_wav;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};

use ringbuf::{
    traits::{Consumer, Producer, Split},
    HeapRb,
};

const AMPLIFICATION: f32 = 100.0;
const LATENCY_TIME_MS: f32 = 1000f32;

fn encode_decode(input: Vec<f32>) -> Vec<f32> {
    let window_size = 1024u32;
    mdct::process_with_tdac(&input, window_size)
}

fn main() {
    let host = cpal::default_host();

    let speaker = host
        .default_output_device()
        .expect("no output device available");

    let mic = host
        .default_input_device()
        .expect("no input device available");

    let mut supported_configs_range = speaker
        .supported_output_configs()
        .expect("error while querying configs");

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let config: StreamConfig = mic.default_input_config().unwrap().into();

    let latency_frames = (LATENCY_TIME_MS / 1000.0) * config.sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * config.channels as usize;

    let ring = HeapRb::<f32>::new(latency_samples * 2);

    let runtime_seconds = 5.0;

    let all_input = Arc::new(Mutex::new(Vec::with_capacity(
        (runtime_seconds * config.sample_rate.0 as f32) as usize,
    )));

    let input_copy = all_input.clone();

    let (mut producer, mut consumer) = ring.split();
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        producer.try_push(0.0).unwrap();
    }

    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let mut output_fell_behind = false;

        let amplified = data.iter().map(|x| x * AMPLIFICATION);

        for sample in amplified {
            input_copy.lock().unwrap().push(sample);
            if producer.try_push(sample).is_err() {
                output_fell_behind = true;
            }
        }
        if output_fell_behind {
            eprintln!("output stream fell behind: try increasing latency");
        }
    };

    let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        let mut input_fell_behind = false;
        for sample in data {
            *sample = match consumer.try_pop() {
                Some(s) => s,
                None => {
                    input_fell_behind = true;
                    0.0
                }
            };
        }
        if input_fell_behind {
            eprintln!("input stream fell behind: try increasing latency");
        }
    };

    // Build streams.
    println!(
        "Attempting to build both streams with f32 samples and `{:?}`.",
        config
    );
    let input_stream = mic
        .build_input_stream(&config, input_data_fn, err_fn, None)
        .unwrap();
    let output_stream = speaker
        .build_output_stream(&config, output_data_fn, err_fn, None)
        .unwrap();
    println!("Successfully built streams.");

    // Play the streams.
    println!(
        "Starting the input and output streams with `{}` milliseconds of latency.",
        LATENCY_TIME_MS
    );
    input_stream.play().unwrap();
    output_stream.play().unwrap();

    // Run for 3 seconds before closing.
    // println!("Playing for 3 seconds... ");
    std::thread::sleep(std::time::Duration::from_secs(runtime_seconds as u64));
    drop(input_stream);
    drop(output_stream);

    let raw_input = all_input.lock().unwrap().to_vec();

    let decoded = encode_decode(raw_input.clone());

    let raw_fp: &Path = &Path::new("raw_input.wav");
    let decoded_fp: &Path = &Path::new("decoded.wav");

    wavers::write(
        raw_fp,
        &*raw_input,
        config.sample_rate.0 as i32,
        config.channels,
    );

    wavers::write(
        decoded_fp,
        &*decoded,
        config.sample_rate.0 as i32,
        config.channels,
    )
    .unwrap();

    println!("Done!");
    // Ok(())
}
