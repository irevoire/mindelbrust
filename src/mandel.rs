use minecrust::packets::play::block::Block::{self, *};

const BLOCKS: [Block; 60] = [
    RedConcrete,
    RedConcretePowder,
    RedWool,
    RedStainedGlass,
    PinkConcrete,
    PinkConcretePowder,
    PinkWool,
    PinkStainedGlass,
    PurpleConcrete,
    PurpleConcretePowder,
    PurpleWool,
    PurpleStainedGlass,
    MagentaConcrete,
    MagentaConcretePowder,
    MagentaWool,
    MagentaStainedGlass,
    OrangeConcrete,
    OrangeConcretePowder,
    OrangeWool,
    OrangeStainedGlass,
    YellowConcrete,
    YellowConcretePowder,
    YellowWool,
    YellowStainedGlass,
    LimeConcrete,
    LimeConcretePowder,
    LimeWool,
    LimeStainedGlass,
    GreenConcrete,
    GreenConcretePowder,
    GreenWool,
    GreenStainedGlass,
    LightBlueConcrete,
    LightBlueConcretePowder,
    LightBlueWool,
    LightBlueStainedGlass,
    BlueConcrete,
    BlueConcretePowder,
    BlueWool,
    BlueStainedGlass,
    CyanConcrete,
    CyanConcretePowder,
    CyanWool,
    CyanStainedGlass,
    LightGrayConcrete,
    LightGrayConcretePowder,
    LightGrayWool,
    LightGrayStainedGlass,
    GrayConcrete,
    GrayConcretePowder,
    GrayWool,
    GrayStainedGlass,
    BlackConcrete,
    BlackConcretePowder,
    BlackWool,
    BlackStainedGlass,
    BrownWool,
    BrownStainedGlass,
    BrownConcrete,
    BrownConcretePowder,
];

pub fn compute(x: i32, y: i32) -> Block {
    let (x1, y1) = (-0.925, 0.266);
    let max_iter = BLOCKS.len();
    let zoom = 300.;

    let c_r = x as f64 / zoom + x1;
    let c_i = y as f64 / zoom + y1;
    let mut z_r = 0.0;
    let mut z_i = 0.0;
    let mut i = 0;

    while (z_r * z_r + z_i * z_i < 4.0) && i < max_iter {
        let tmp = z_r;
        z_r = z_r * z_r - z_i * z_i + c_r;
        z_i = 2.0 * z_i * tmp + c_i;
        i += 1;
    }

    if i == max_iter {
        return Block::WhiteConcrete;
    } else {
        let idx = BLOCKS.len() - i;
        return BLOCKS[idx];
    }
}
