use super::*;
use std::ops::{Index, IndexMut};

pub const GFX_MEMORY_SIZE: usize = renderer::DISPLAY_HEIGHT * renderer::DISPLAY_WIDTH;

pub struct GfxMemory {
    memory: [u8; GFX_MEMORY_SIZE],
}

impl Index<usize> for GfxMemory {
    type Output = u8;
    fn index(&self, i: usize) -> &u8 {
        &self.memory[i]
    }
}

impl IndexMut<usize> for GfxMemory {
    fn index_mut(&mut self, i: usize) -> &mut u8 {
        &mut self.memory[i]
    }
}

impl GfxMemory {
    pub fn new() -> GfxMemory {
        GfxMemory { memory: [0; GFX_MEMORY_SIZE] }
    }

    pub fn clear(&mut self) {
        self.memory = [0; GFX_MEMORY_SIZE];
    }

    #[cfg(test)]
    pub fn get(&self) -> &[u8] {
        &self.memory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gfx_clear() {
        let mut gfx_memory = GfxMemory::new();
        gfx_memory.clear();
        assert!(gfx_memory.get().iter().all(|&x| x == 0));
    }

    #[test]
    fn gfx_get_set_pixel() {
        let mut gfx_memory = GfxMemory::new();
        gfx_memory[0] = 0xFF;
        assert_eq!(gfx_memory[0], 0xFF);
    }
}