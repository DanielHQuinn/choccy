use crate::emulator::Emu;

/// The `Graphics` trait represents the graphics capabilities of the CHIP-8 emulator.
pub trait Graphics {
    /// Clears the screen, which is represented by Graphics.
    fn clear(&mut self);

    /// Draws a sprite at the given x and y coordinates.
    fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool;

    /// Returns the height of the screen.
    fn height(&self) -> u8;

    /// Returns the width of the screen.
    fn width(&self) -> u8;
}

// TODO: decide if instead of implementing Graphics for Emu,
// we should implement Graphics for a struct that wraps Emu
// and provides a mutable reference to it.
impl Graphics for Emu {
    fn clear(&mut self) {
        todo!()
    }

    fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        todo!()
    }

    fn height(&self) -> u8 {
        todo!()
    }

    fn width(&self) -> u8 {
        todo!()
    }
}
