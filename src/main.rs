use risc0_core::field::{
    Elem,
    baby_bear::BabyBearElem,                        
};
use rand::thread_rng;

fn main() {
    let a = BabyBearElem::random(&mut thread_rng());
    let b = BabyBearElem::random(&mut thread_rng());
    
    let a_usize = a.as_u32().try_into().unwrap();
    let b_usize = b.as_u32().try_into().unwrap();
    
    let g = BabyBearElem::new(7);
    
    // g to the power of a_u32*b_u32.
    let g_ab_usize = g.pow(a_usize*b_usize);
    
    // g to the power of a*b
    let g_ab = g.pow((a*b).as_u32().try_into().unwrap());
    
    // g to the power of a, to the power of b.
    let g_a_b = g.pow(a_usize).pow(b_usize);
    
    assert_eq!(g_ab_usize, g_a_b);
    assert_ne!(g_ab, g_a_b);
}
