//! Utilities for encoding symbols as a C4FM signal.

use bits;
use consts;

/// Yields a series of scaled impulses vs time corresponding to given dibits.
pub struct C4fmImpulses<T> {
    /// The dibit source to iterate over.
    src: T,
    /// Current global sample index.
    sample: usize,
}

impl<T: Iterator<Item = bits::Dibit>> C4fmImpulses<T> {
    /// Construct a new `C4fmImpulses<T>` from the given source and sample rate.
    pub fn new(src: T) -> C4fmImpulses<T> {
        C4fmImpulses {
            src: src,
            sample: 0,
        }
    }
}

impl<T: Iterator<Item = bits::Dibit>> Iterator for C4fmImpulses<T> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        // Store current sample and move to the next.
        let s = self.sample;
        self.sample += 1;

        // Impulse is only output at the beginning of a symbol period.
        if s % consts::SYMBOL_PERIOD != 0 {
            return Some(0.0);
        }

        // Map the current dibit to a scaled impulse.
        if let Some(dibit) = self.src.next() {
            match dibit.bits() {
                0b01 => Some(0.18),
                0b00 => Some(0.06),
                0b10 => Some(-0.06),
                0b11 => Some(-0.18),
                _ => unreachable!(),
            }
        } else {
            None
        }
    }
}

/// Generates the alternating series of dibits used for the C4FM deviation test. The
/// resulting filtered waveform approximates a 1200Hz sine wave.
pub struct C4fmDeviationDibits {
    /// Used to alternate dibits.
    idx: usize,
}

impl C4fmDeviationDibits {
    /// Construct a new `C4fmDeviationDibits`.
    pub fn new() -> C4fmDeviationDibits {
        C4fmDeviationDibits {
            idx: 0,
        }
    }
}

impl Iterator for C4fmDeviationDibits {
    type Item = bits::Dibit;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;

        self.idx += 1;
        self.idx %= 4;

        Some(if idx < 2 {
            bits::Dibit::new(0b01)
        } else {
            bits::Dibit::new(0b11)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bits;

    #[test]
    fn test_impulses() {
        const BITS: &'static [u8] = &[
            0b00011011,
        ];

        let d = bits::Dibits::new(BITS.iter().cloned());
        let mut imp = C4fmImpulses::new(d);

        assert!(imp.next().unwrap() == 0.06);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.18);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == -0.06);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == -0.18);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().unwrap() == 0.0);
        assert!(imp.next().is_none());
    }

    #[test]
    fn test_deviation() {
        let mut d = C4fmDeviationDibits::new();

        assert!(d.next().unwrap().bits() == 0b01);
        assert!(d.next().unwrap().bits() == 0b01);
        assert!(d.next().unwrap().bits() == 0b11);
        assert!(d.next().unwrap().bits() == 0b11);
        assert!(d.next().unwrap().bits() == 0b01);
        assert!(d.next().unwrap().bits() == 0b01);
        assert!(d.next().unwrap().bits() == 0b11);
        assert!(d.next().unwrap().bits() == 0b11);
    }
}
