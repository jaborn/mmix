use machine::state::State;

pub fn sri(state: &mut State, x: u8, y: u8, z: u8) {
    // Load operand
    let op1: i64 = state.gpr[y].into();

    // Execute
    let res = op1.overflowing_shr(z as u32).0;

    // Store results
    state.gpr[x] = res.into();
}
