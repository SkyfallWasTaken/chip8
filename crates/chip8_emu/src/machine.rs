use ndarray::Array2;

use crate::font;

/// Most emulators expect a game to be loaded into memory after it,
/// starting at address `0x200` (512 in decimal).
///
/// We also start at `0x200` for compatibility with games that expect
/// themselves to start there.
pub const GAME_MEM_START: usize = 0x200;

/// The CHIP-8 machine emulator. Contains memory, display etc.
pub struct Machine {
    /// Writeable memory - 4096 bytes.
    ///
    /// Addresses start from `000` to `1FF`.
    /// A CHIP-8 game should be loaded from `machine::GAME_MEM_START`.
    pub memory: Vec<u8>,

    /// A 64x32 pixel display - each pixel is either _on_ or _off_.
    pub display: Array2<bool>,

    /// Stack of 16-bit numbers
    // TODO: not meant to be 16-bit! Meant to be 12-bit? Use 12bit crate?
    pub stack: Vec<u16>,

    /// The delay timer decrements 60 times a second, till it reaches zero.
    ///
    /// **The program runs as normal even if the delay timer is above zero!**
    /// It is the program's responsibility to check the timer and delay itself
    /// if it needs to.
    pub delay_timer: u8,

    /// The sound timer decrements 60 times a second, till it reaches zero.
    ///
    /// When the value of the timer is above zero, the machine should beep until it reaches zero.
    /// Use the `should_beep` method to check if sound is required:
    ///
    /// ```rust
    /// use chip8_emu::machine::Machine;
    ///
    /// let machine = Machine::default();
    /// // ... load game, initialize execution, etc.
    /// // this should run in the execution loop
    /// if machine.should_beep() {
    ///     println!("Beep!")
    /// }
    /// ```
    pub sound_timer: u8,
}

impl Default for Machine {
    fn default() -> Self {
        let mut machine = Self {
            memory: vec![0; 4096],
            // TODO: remove magic number
            display: Array2::<bool>::from_elem((64, 32), false),
            stack: Vec::with_capacity(16), // 16 is usually enough for most games
            delay_timer: 0,
            sound_timer: 0,
        };

        // Let's copy the font into memory
        // TODO: optimize?
        machine.memory[font::START..font::START + font::VALUES.len()]
            .copy_from_slice(&font::VALUES);

        machine
    }
}

impl Machine {
    // TODO: consider using slices instead
    // TODO: docs
    /// Takes a `Vec<u8>` of game bytes, loads it into memory, and returns the machine.
    ///
    /// ## Example
    /// ```ignore
    /// use chip8_emu::machine::Machine;
    ///
    /// let machine = Machine::default()
    ///     .load_game(std::fs::read("./roms/ch8_logo.ch8").unwrap());
    /// ```
    pub fn load_game(&mut self, game: Vec<u8>) -> &mut Machine {
        self.memory[GAME_MEM_START..GAME_MEM_START + game.len()].copy_from_slice(&game);
        self
    }

    /// An utility function to check if the emulator should beep
    /// or otherwise make a sound.
    ///
    /// This works by checking if the sound timer is above zero.
    pub fn should_beep(&self) -> bool {
        self.sound_timer > 0
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

    #[test]
    fn timers_start_at_zero() {
        let machine = Machine::default();
        assert_eq!(machine.delay_timer, 0, "delay timer should be zero");
        assert_eq!(machine.sound_timer, 0, "sound timer should be zero");
    }
}
