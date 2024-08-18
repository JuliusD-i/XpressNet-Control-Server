

struct Locomotive {
    id: u16,
}

pub struct RailwaySetupState {
    id: u32,
    emergency_off: bool,
    locos: Vec<Locomotive>
    //TODO: Add the rest here
}