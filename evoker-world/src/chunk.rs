//! Chunk representation

use serde::{Deserialize, Serialize};

/// Chunk coordinate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkCoord {
    pub x: i32,
    pub z: i32,
}

impl ChunkCoord {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}

/// Block ID type
pub type BlockId = u16;

/// Chunk dimensions
pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_DEPTH: usize = 16;

/// A chunk of blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    coord: ChunkCoord,
    blocks: Vec<BlockId>,
    modified: bool,
}

impl Chunk {
    /// Create a new empty chunk
    pub fn new(coord: ChunkCoord) -> Self {
        let total_blocks = CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH;
        Self {
            coord,
            blocks: vec![0; total_blocks], // 0 = air
            modified: false,
        }
    }
    
    /// Generate a new chunk with terrain
    pub fn generate(coord: ChunkCoord, seed: i64) -> Self {
        let mut chunk = Self::new(coord);
        
        // Use Perlin noise for terrain generation
        use noise::{NoiseFn, Perlin, Seedable};
        
        let perlin = Perlin::new(seed as u32);
        let scale = 0.05; // Controls terrain smoothness
        let height_multiplier = 32.0; // Controls terrain height variation
        let base_height = 64; // Sea level
        
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_DEPTH {
                // Calculate world coordinates
                let world_x = (coord.x * CHUNK_WIDTH as i32 + x as i32) as f64;
                let world_z = (coord.z * CHUNK_DEPTH as i32 + z as i32) as f64;
                
                // Sample noise for height
                let noise_value = perlin.get([world_x * scale, world_z * scale]);
                let height = (base_height as f64 + noise_value * height_multiplier) as usize;
                let height = height.min(CHUNK_HEIGHT - 1);
                
                // Generate terrain layers
                for y in 0..=height {
                    let block_id = if y == 0 {
                        1 // Bedrock at bottom
                    } else if y < height.saturating_sub(4) {
                        2 // Stone
                    } else if y < height {
                        3 // Dirt
                    } else {
                        // Top layer depends on height
                        if height < base_height - 5 {
                            7 // Sand (below sea level)
                        } else if height < base_height + 20 {
                            4 // Grass
                        } else if height < base_height + 50 {
                            2 // Stone (mountains)
                        } else {
                            6 // Snow (mountain peaks)
                        }
                    };
                    chunk.set_block(x, y, z, block_id);
                }
                
                // Fill water below sea level
                if height < base_height {
                    for y in (height + 1)..base_height {
                        chunk.set_block(x, y, z, 5); // Water
                    }
                }
            }
        }
        
        chunk
    }
    
    /// Get block at position
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<BlockId> {
        if x >= CHUNK_WIDTH || y >= CHUNK_HEIGHT || z >= CHUNK_DEPTH {
            return None;
        }
        
        let index = x + z * CHUNK_WIDTH + y * CHUNK_WIDTH * CHUNK_DEPTH;
        self.blocks.get(index).copied()
    }
    
    /// Set block at position
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block_id: BlockId) -> bool {
        if x >= CHUNK_WIDTH || y >= CHUNK_HEIGHT || z >= CHUNK_DEPTH {
            return false;
        }
        
        let index = x + z * CHUNK_WIDTH + y * CHUNK_WIDTH * CHUNK_DEPTH;
        if let Some(block) = self.blocks.get_mut(index) {
            *block = block_id;
            self.modified = true;
            true
        } else {
            false
        }
    }
    
    /// Get chunk coordinate
    pub fn coord(&self) -> ChunkCoord {
        self.coord
    }
    
    /// Check if chunk has been modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }
    
    /// Mark chunk as saved
    pub fn mark_saved(&mut self) {
        self.modified = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chunk_creation() {
        let coord = ChunkCoord::new(0, 0);
        let chunk = Chunk::new(coord);
        
        assert_eq!(chunk.coord(), coord);
        assert_eq!(chunk.get_block(0, 0, 0), Some(0));
    }
    
    #[test]
    fn test_chunk_block_operations() {
        let coord = ChunkCoord::new(0, 0);
        let mut chunk = Chunk::new(coord);
        
        assert!(chunk.set_block(5, 10, 5, 42));
        assert_eq!(chunk.get_block(5, 10, 5), Some(42));
        assert!(chunk.is_modified());
    }
    
    #[test]
    fn test_chunk_generation() {
        let coord = ChunkCoord::new(0, 0);
        let chunk = Chunk::generate(coord, 12345);
        
        // Should have some non-air blocks
        let block = chunk.get_block(8, 64, 8);
        assert!(block.is_some());
        assert_ne!(block.unwrap(), 0);
    }
}
