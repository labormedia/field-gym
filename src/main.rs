use risc0_core::field::{
    Elem,
    baby_bear::BabyBearElem,                        
};
use rand::thread_rng;

fn main() {
    let one = BabyBearElem::ONE;
    let zero = BabyBearElem::ZERO;
    
    let a = BabyBearElem::random(&mut thread_rng());
    let b = BabyBearElem::random(&mut thread_rng());
    
    let a_add_inv = zero - a;
    let b_add_inv = zero - b;
    
    // a and b usize representation
    let a_usize: usize = a.as_u32().try_into().unwrap();
    let b_usize:usize  = b.as_u32().try_into().unwrap();
    
    // a_add_inv and b_add_inv usize representation
    let a_add_inv_usize: usize = a_add_inv.as_u32().try_into().unwrap();
    let b_add_inv_usize: usize = b_add_inv.as_u32().try_into().unwrap();
    
    let g = BabyBearElem::new(7);
    
    // g to the power of a_usize and b_usize
    let g_a = g.pow(a_usize);
    let g_b = g.pow(b_usize);
    
    // g to the power of a_add_inv_usize and b_add_inv_usize
    let g_a_add_inv = g.pow(a_add_inv_usize);
    let g_b_add_inv = g.pow(b_add_inv_usize);
    
    // g to the power of a_u32*b_u32.
    let g_ab_usize = g.pow(a_usize*b_usize);
    
    // g to the power of a*b
    let g_ab = g.pow((a*b).as_u32().try_into().unwrap());
    
    // g to the power of a, to the power of b.
    let g_a_b = g.pow(a_usize).pow(b_usize);
    
    assert_eq!(g_ab_usize, g_a_b);
    assert_ne!(g_ab, g_a_b);
    assert_eq!(a + a_add_inv, zero);
    assert_ne!(g_a*g_a_add_inv, one);
    assert_eq!(g_a*g_a_add_inv, g);
}
