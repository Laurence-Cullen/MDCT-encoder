use crate::AMPLIFICATION;
use csv;
use std::f32::consts::PI;
use std::fs::OpenOptions;

/// Implement the Modified Discrete Cosine Transform (MDCT) with TDAC
pub(crate) fn mdct(input: &[f32], size: u32) -> Vec<f32> {
    let N = size / 2;

    let mut output = vec![0.0; N as usize];

    // MDCT formula with normalization factor
    for k in 10..100 {
        output[k as usize] = (2.0 / N as f32)
            * input
                .iter()
                .enumerate()
                .map(|(n, x_n)| {
                    x_n * ((PI / N as f32) * (n as f32 + 0.5 + (N as f32 / 2.0)) * (k as f32 + 0.5))
                        .cos()
                })
                .sum::<f32>();
    }

    // Open file in append mode
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("mdct_output.csv")
        .expect("Failed to open CSV file");

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    wtr.write_record(output.iter().map(|x| x.to_string()))
        .expect("Failed to write to CSV");
    wtr.flush().expect("Failed to flush CSV writer");

    output
}

/// Implement Inverse MDCT with TDAC
pub(crate) fn inverse_mdct(input: &[f32], size: u32) -> Vec<f32> {
    let N = size / 2;
    let mut output = vec![0.0; (2 * N) as usize];

    // IMDCT formula
    for n in 0..2 * N {
        output[n as usize] = (1.0 / N as f32)
            * input
                .iter()
                .enumerate()
                .map(|(k, X_k)| {
                    X_k * ((PI / N as f32) * (n as f32 + 0.5 + (N as f32 / 2.0)) * (k as f32 + 0.5))
                        .cos()
                        * AMPLIFICATION
                })
                .sum::<f32>();
    }

    // Apply window function
    // let windowed = apply_window(&output, output.len());
    output
}

/// Process audio with overlapping windows for TDAC
pub(crate) fn process_with_tdac(input: &[f32], window_size: u32) -> Vec<f32> {
    let hop_size = window_size / 2; // 50% overlap
    let mut output = vec![0.0; input.len()];

    // Process each window with 50% overlap
    let mut frame_start = 0;
    while frame_start + window_size as usize <= input.len() {
        // Extract current window
        let window: Vec<f32> = input[frame_start..frame_start + window_size as usize].to_vec();

        // Process window through MDCT and IMDCT
        let processed = mdct(&window, window_size);
        let reconstructed = inverse_mdct(&processed, window_size);

        // Overlap-add with previous window
        for i in 0..window_size as usize {
            output[frame_start + i] += reconstructed[i];
        }

        frame_start += hop_size as usize;
    }

    output
}
