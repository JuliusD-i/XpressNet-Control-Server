pub enum SwitchTypes {
    ReceiverNoFeedback,
    ReceiverFeedback,
    FeedbackComponent,
    Undefined,
}

pub enum SwitchStates {
    NotYetSwitchedOrUnknown,
    SwitchedLeft,
    SwitchedRight,
    Undefined,
}

struct Locomotive {
    address: u16,
    drive_level_count: u8,
    direction_forward: bool,
    current_drive_level: u8,
    function_state: Vec<bool>,
    is_in_mtr: bool,
    mtr_address: u8,
}

struct Switch {
    address: u16,
    currently_switching: bool,
    switchtype: SwitchTypes,
    state: SwitchStates,
}

pub struct RailwaySetupState {
    controller_id: i8,
    lz_version: f32,
    emergency_off: bool,
    loco_off: bool,
    locos: Vec<Locomotive>,
    switches: Vec<Switch>,
    //TODO: Add the rest here
}

impl RailwaySetupState {
    pub fn new() -> RailwaySetupState {
        RailwaySetupState {
            controller_id: -1,
            lz_version: 0.0,
            emergency_off: false,
            locos: vec![],
            switches: vec![],
            loco_off: false,
        }
    }
}
