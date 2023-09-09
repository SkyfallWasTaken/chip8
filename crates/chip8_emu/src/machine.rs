use ndarray::Array2;

use crate::font;

/// The CHIP-8 machine emulator. Contains memory, display etc.
pub struct Machine {
    /// Writeable memory - 4096 bytes.
    ///
    /// Addresses start from `000` to `1FF`.
    /// A CHIP-8 program starts at `200`.
    pub memory: Vec<u8>,

    /// A 64x32 pixel display - each pixel is either _on_ or _off_.
    pub display: Array2<bool>,
}

impl Default for Machine {
    fn default() -> Self {
        let mut machine = Self {
            memory: vec![0; 4096],
            display: Array2::<bool>::from_elem((64, 32), false),
        };

        // Let's copy the font into memory
        // TODO: optimize?
        machine.memory[font::START..font::START + font::VALUES.len()]
            .copy_from_slice(&font::VALUES);

        machine
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn memory_length_is_4_kib() {
        let machine = Machine::default();
        assert_eq!(machine.memory.len(), 4096);
    }

    #[test]
    fn font_is_copied_into_memory() {
        let machine = Machine::default();
        // TODO: check more exhaustively
        assert_eq!(machine.memory[font::START], font::VALUES[0])
    }

    #[test]
    fn display_pixels_start_as_disabled() {
        let machine = Machine::default();
        for item in machine.display {
            assert_eq!(item, false)
        }
    }
}
