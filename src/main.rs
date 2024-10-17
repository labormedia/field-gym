use risc0_core::field::{
    Elem,
    baby_bear::BabyBearElem,                        
};
use rand::thread_rng;

fn main() {
    let a = BabyBearElem::random(&mut thread_rng());
    let b = BabyBearElem::random(&mut thread_rng());
    
    let a_u32 = a.as_u32().try_into().unwrap();
    let b_u32 = b.as_u32().try_into().unwrap();
    
    let g = BabyBearElem::new(7);
    
    // g to the power of a_u32*b_u32.
    let g_ab = g.pow(a_u32*b_u32);
    
    // g to the power of a, to the power of b.
    let g_a_b = g.pow(a_u32).pow(b_u32);
    
    assert_eq!(g_ab, g_a_b);
}
