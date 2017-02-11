use machine::state::State;
use machine::state::sr::R;

pub fn feqle(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: f64 = state.gpr[y].into();
    let op2: f64 = state.gpr[z].into();
    let eps: f64 = state.sr[R::E].into();

    // Execute
    let res = ((op1 as i64).wrapping_sub(op2 as i64)).abs();

    // Store result
    if res <= eps as i64 {
        state.gpr[x] = 1u64.into();
    } else {
        state.gpr[x] = 0u64.into();
    }
}
