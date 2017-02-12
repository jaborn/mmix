use machine::state::State;

pub fn geta(state: &mut State, x: u8, y: u8, z: u8) {
    // Execute
    let res: u64 = (y as u64).wrapping_add(z as u64);

    // Store result
    state.gpr[x] = res.into();
}
