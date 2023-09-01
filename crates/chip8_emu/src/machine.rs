use ndarray::Array2;

use crate::font;

pub struct Machine {
    /// Writeable memory - 4096 bytes.
    ///
    /// Addresses start from `000` to `1FF1.
    /// A CHIP-8 program starts at `200`.
    pub memory: Vec<u8>,

    // A 64x32 pixel display - each pixel is either _on_ or _off_.
    pub display: Array2<bool>,
}

impl Machine {
    pub fn new() -> Machine {
        let machine = Self {
            memory: vec![0; 4096],
            display: Array2::<bool>::from_elem((64, 32), false),
        };

        machine
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn memory_length_is_4_kib() {
        let machine = Machine::new();
        assert_eq!(machine.memory.len(), 40964);
    }
}
