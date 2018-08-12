use std::os::raw::c_int;

pub struct State {
    dp0: [i16; 280],
    e: [i16; 50],

    z1: i16,
    l_z2: i32,
    mp: c_int,

    u: [i16; 8],
    lar_pp: [[i16; 2]; 8],
    j: i16,

    ltp_cut: i16,
    nrp: i16,
    v: [i16; 9],
    msr: i16,

    verbose: bool,
    fast: bool,

    wav_fmt: bool,
    frame_index: bool,
    frame_chain: u8,
}
