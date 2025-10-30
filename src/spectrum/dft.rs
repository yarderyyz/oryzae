use num::Complex;
use std::f32::consts::PI;

enum DftDirection {
    Forward,
    Inverse,
}

// Naive DFT
pub fn dft(in_frame: &[f32]) -> Vec<Complex<f32>> {
    let complex_frame: Vec<_> = in_frame.iter().map(|&x| Complex::new(x, 0.0)).collect();
    kernel(&complex_frame, 1.0, DftDirection::Forward)
}

pub fn idft(in_frame: &[Complex<f32>]) -> Vec<f32> {
    let n = in_frame.len();
    let normalization = 1.0 / n as f32;

    kernel(in_frame, normalization, DftDirection::Inverse)
        .iter()
        .map(|c| c.re)
        .collect()
}

/// DFT formula: X[j] = Σ(k=0 to N-1) x[k] * e^(i*th*j*k)
fn kernel(
    in_frame: &[Complex<f32>],
    normalization: f32,
    direction: DftDirection,
) -> Vec<Complex<f32>> {
    let n = in_frame.len();
    let th = match direction {
        DftDirection::Forward => 2.0 * PI / n as f32,
        DftDirection::Inverse => -2.0 * PI / n as f32,
    };
    (0..in_frame.len())
        .map(|j| {
            let mut sum = in_frame
                .iter()
                .enumerate()
                .map(|(k, &val)| {
                    // w = e^(i*th*j*k)
                    let w = Complex::from_polar(1.0, th * (j * k) as f32);
                    w * val
                })
                .sum::<Complex<f32>>();
            sum *= normalization;
            sum
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_out_dims() {
        let in_frame = [1.0f32];
        let out = dft(&in_frame);
        assert_eq!(out.len(), 1);

        let in_frame = [1.0f32, 1.0f32];
        let out = dft(&in_frame);
        assert_eq!(out.len(), 2);

        let in_frame = [1.0f32, 1.0f32, 1.0f32, 1.0f32];
        let out = dft(&in_frame);
        assert_eq!(out.len(), 4);
    }

    #[test]
    fn test_size_1() {
        let in_frame = [1.0f32];
        let out = dft(&in_frame);
        assert_eq!(out, vec![Complex::new(1.0, 0.0)]);
    }

    #[test]
    fn test_size_2() {
        let in_frame = [1.0f32, 1.0f32];
        let out = dft(&in_frame);
        assert_relative_eq!(out[0].re, 2.0, epsilon = 1e-6);
        assert_relative_eq!(out[0].im, 0.0, epsilon = 1e-6);
        assert_relative_eq!(out[1].re, 0.0, epsilon = 1e-6);
        assert_relative_eq!(out[1].im, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_inv() {
        let in_frame = [1.0f32, 1.0f32];
        let dft_out = dft(&in_frame);
        let idft_out = idft(dft_out.as_slice());
        assert_eq!(in_frame, idft_out.as_slice());
    }
}
