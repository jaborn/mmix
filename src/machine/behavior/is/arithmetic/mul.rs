use machine::state::State;

pub fn mul(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operands
    let op1: i64 = state.gpr[y].into();
    let op2: i64 = state.gpr[z].into();

    // Execute
    let res = op1.wrapping_mul(op2);

    // Store result
    state.gpr[x] = res.into();
}
