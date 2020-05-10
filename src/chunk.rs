use minecrust::game::map::generator::SingleBlockGenerator;
use minecrust::packets::play::block::Block;

pub struct MandelbrotGenerator();

impl SingleBlockGenerator for MandelbrotGenerator {
    fn block(&self, x: i32, y: u16, z: i32) -> Option<Block> {
        if y >= 1 {
            return None;
        }
        Some(crate::mandel::compute(x, z))
    }
}
