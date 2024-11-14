use std::f32::consts::PI;

/// Implement the Modified Discrete Cosine Transform (MDCT) algorithm.
pub(crate) fn mdct(input: &[f32], size: u32) -> Vec<f32> {
    let N = size / 2;
    let mut output: Vec<f32> = vec![0.0; N as usize];

    for k in 0..N {
        output[k as usize] = input
            .iter()
            .enumerate()
            .map(|(n, x_n)| {
                x_n * ((PI / N as f32) * (n as f32 + 0.5 + (N as f32 / 2.0)) * (k as f32 + 0.5))
                    .cos()
            })
            .sum();
    }

    output
}

pub(crate) fn inverse_mdct(input: &[f32], size: u32) -> Vec<f32> {
    let N = size;
    let mut output = vec![0.0; (N * 2u32) as usize];

    for n in 0..2 * N {
        output[n as usize] = input
            .iter()
            .enumerate()
            .map(|(k, X_k)| {
                X_k * ((PI / N as f32) * (n as f32 + 0.5 + (N as f32/ 2.0)) * (k as f32+ 0.5)).cos()
            })
            .sum();
    }
    output
}
