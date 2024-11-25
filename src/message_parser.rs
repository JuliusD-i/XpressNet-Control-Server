#[allow(unused_variables)]
#[derive(Debug, Eq, PartialEq)]
pub enum CallTypes {
    // P: Parity bit, 1 if an even amount of ones are in the call byte
    // A: 5 bit address of the peer
    SendRequest,       //P10A AAAA -> Plese send me a message if you have one
    RequestResend,     //P00A AAAA -> Please resend last message
    Message,           //P11A AAAA -> Expect a header byte and more data
    BroadcastFeedback, // P010 0000
    Unknown,           // P01A AAAA -> Unknown. Reserved for later expansions by the manufacturer
}
#[derive(Debug, Eq, PartialEq)]
pub struct MessageCallType {
    message_type: CallTypes,
    target_address: u8,
    raw_value: u8,
    receive_time_picosec: u64,
    is_broadcast: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub struct HeaderType {
    name: String,
    raw_value: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PossibleMessages {
    header: String[],
    raw_data: u8,
}




#[derive(Debug, Eq, PartialEq)]
pub enum MessageFromMaster {
    SendRequest {
        message_call_type: MessageCallType,
    },
    ResendRequest {
        message_call_type: MessageCallType,
    },
    BroadCastEmergencyOff {
        // Header: 0b0110 0001
        message_call_type: MessageCallType,
        all_off: bool,
    },
    BroadcastHaltAll {
        // Header: 0b1000 0001
        message_call_type: MessageCallType,
        all_stop: bool,
    },
    BroadcastFeedback {
        // Header: 0b0100 NNNN (N: Number of following data bytes
        message_call_type: MessageCallType,
        bytecount: u8,
        data: Vec<u8>,
    },
    // TODO: Programming mode stuff
    SoftwareVersion {
        // Header: 0b0110 0010 for version 2.3 and below. 0b0110 0011 from version 3.0
        message_call_type: MessageCallType,
        major_version: u8,
        minor_version: u8,
        master_type: u8, //0x00: LZ 100, 0x01 LH200,0x02 DPC - Zentrale (Compact and Commander)
    },
    StatusRequest {
        // Header: 0b0110 0010 Same as version. Gets differentiated from first data byte.
        message_call_type: MessageCallType,
        emergency_off: bool,
        emergency_halt: bool,
        startmode_automatic: bool,
        programming_mode: bool,
        master_cold_start: bool,
        ram_check_error: bool,
        data_on_rail: bool,
    },
    TransmitError {
        // Header: 0b0110 0001
        message_call_type: MessageCallType,
    },
    MasterBusy {
        // Header: 0b0110 0001
        message_call_type: MessageCallType,
    },
    CommandUnknown {
        // Header: 0b0110 0001
        message_call_type: MessageCallType,
    },
    SwitchState {
        // Header:0b0100 0010
        message_call_type: MessageCallType,
        switch_address_raw: u16,
        switch_moving: bool,
        receiver_type: u8,
        upper_lower_nibble: bool,
        raw_data: Vec<u8>,
    },
    LocoInfo15 {
        // LocoInfo for up to master version 1.5
        // Header: 0b1000 0011 for master version up to 1.5, 0b1010 0011 if it is reserved by another controller
        message_call_type: MessageCallType,
        loco_address: u8,
        loco_reserved_by_other_controller: bool,
        direction_forward: bool,
        current_drive_level: u8, // Values between 0 and 16, 0 is stop, 1 is emergency off, values are drive level from 1 to 14
        function_states: [bool; 5],
    },
    LocoInfo23 {
        // Header: 0b1000 0100 if not reserved, 0b1010 0100
        message_call_type: MessageCallType,
        loco_address: u8,
        loco_reserved_by_other_controller: bool,
        drive_levels: u8, // How many drive levels are configured for this loco
        direction_forward: bool,
        current_drive_level: u8,
        function_states: [bool; 5],
    },
    LocoInfo30 {
        // Header: 0b1110 0100
        message_call_type: MessageCallType,
        loco_address: u8,
        loco_reserved_by_other_controller: bool,
        direction_forward: bool,
        current_drive_level: u8, // Values between 0 and 16, 0 is stop, 1 is emergency off, values are drive level from 1 to 14
        function_states: [bool; 29],
    },
    // TODO: Implement further. Implemented up to 2.19.1
}

fn get_bit_by_index(num: u8, index: usize) -> bool {
    if index < 8 {
        (num >> index) & 1 == 1
    } else {
        false // or handle the out-of-bounds case as appropriate
    }
}

pub fn parse_call(data: u8, receive_time: u64) -> Result<MessageCallType, String> {
    let ones = data.count_ones();
    let bit_at_index = get_bit_by_index(data, 7);
    let parity_true = bit_at_index == (ones % 2 == 0);

    let call_type = data & 0b01100000;
    let address = data & 0b0001111;
    let broadcast = match address {
        0 => true,
        _ => false,
    };
    if call_type == 0b01000000 && parity_true {
        Ok(MessageCallType {
            message_type: CallTypes::SendRequest,
            target_address: address,
            raw_value: data,
            receive_time_picosec: receive_time,
            is_broadcast: broadcast,
        })
    } else if call_type == 0b00100000 && parity_true && broadcast {
        Ok(MessageCallType {
            message_type: CallTypes::BroadcastFeedback,
            target_address: address,
            raw_value: data,
            receive_time_picosec: receive_time,
            is_broadcast: broadcast,
        })
    } else if call_type == 0b00000000 && parity_true && !broadcast {
        Ok(MessageCallType {
            message_type: CallTypes::RequestResend,
            target_address: address,
            raw_value: data,
            receive_time_picosec: receive_time,
            is_broadcast: broadcast,
        })
    } else if call_type == 0b01100000 && broadcast {
        Ok(MessageCallType {
            message_type: CallTypes::Message,
            target_address: address,
            raw_value: data,
            receive_time_picosec: receive_time,
            is_broadcast: broadcast,
        })
    } else if call_type == 0b01100000 && parity_true && !broadcast {
        Ok(MessageCallType {
            message_type: CallTypes::Message,
            target_address: address,
            raw_value: data,
            receive_time_picosec: receive_time,
            is_broadcast: broadcast,
        })
    } else if call_type == 0b00100000 && parity_true {
        Ok(MessageCallType {
            message_type: CallTypes::Unknown,
            target_address: address,
            raw_value: data,
            receive_time_picosec: receive_time,
            is_broadcast: broadcast,
        })
    } else if !parity_true {
        Err("Parity Error!".parse().unwrap())
    } else {
        Err("Unknown Error!".parse().unwrap())
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_send_request() {
        let data = 0b01000011; // Example data for SendRequest
        let receive_time = 100;
        let result = parse_call(data, receive_time);
        assert!(result.is_ok());

        let message_call_type = result.unwrap();
        assert_eq!(message_call_type.message_type, CallTypes::SendRequest);
        assert_eq!(message_call_type.target_address, 3);
        assert_eq!(message_call_type.raw_value, data);
        assert_eq!(message_call_type.receive_time_picosec, receive_time);
        assert!(!message_call_type.is_broadcast);
    }

    #[test]
    fn test_parse_request_resend() {
        let data = 0b00000001; // Example data for RequestResend
        let receive_time = 200;
        let result = parse_call(data, receive_time);
        assert!(result.is_ok());

        let message_call_type = result.unwrap();
        assert_eq!(message_call_type.message_type, CallTypes::RequestResend);
        assert_eq!(message_call_type.target_address, 1);
        assert_eq!(message_call_type.raw_value, data);
        assert_eq!(message_call_type.receive_time_picosec, receive_time);
        assert!(!message_call_type.is_broadcast);
    }

    #[test]
    fn test_parse_broadcast_feedback() {
        let data = 0b00100000; // Example data for BroadcastFeedback
        let receive_time = 300;
        let result = parse_call(data, receive_time);
        assert!(result.is_ok());

        let message_call_type = result.unwrap();
        assert_eq!(message_call_type.message_type, CallTypes::BroadcastFeedback);
        assert_eq!(message_call_type.target_address, 0);
        assert_eq!(message_call_type.raw_value, data);
        assert_eq!(message_call_type.receive_time_picosec, receive_time);
        assert!(message_call_type.is_broadcast);
    }

    #[test]
    fn test_parse_message() {
        let data = 0b01100010; // Example data for Message
        let receive_time = 400;
        let result = parse_call(data, receive_time);
        assert!(result.is_ok());

        let message_call_type = result.unwrap();
        assert_eq!(message_call_type.message_type, CallTypes::Message);
        assert_eq!(message_call_type.target_address, 2);
        assert_eq!(message_call_type.raw_value, data);
        assert_eq!(message_call_type.receive_time_picosec, receive_time);
        assert!(!message_call_type.is_broadcast);
    }

    #[test]
    fn test_parse_unknown() {
        let data = 0b00100010; // Example data for Unknown type
        let receive_time = 500;
        let result = parse_call(data, receive_time);
        assert!(result.is_ok());

        let message_call_type = result.unwrap();

        assert_eq!(message_call_type.message_type, CallTypes::Unknown);
        assert_eq!(message_call_type.target_address, 2);
        assert_eq!(message_call_type.raw_value, data);
        assert_eq!(message_call_type.receive_time_picosec, receive_time);
        assert!(!message_call_type.is_broadcast);
    }

    #[test]
    fn test_parse_parity_error() {
        let data = 0b01000001; // Example data with parity error
        let receive_time = 600;
        let result = parse_call(data, receive_time);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Parity Error!");
    }

    #[test]
    fn test_get_bit_by_index_valid() {
        let num: u8 = 0b10101100; // Binary: 10101100

        // Test each bit position
        assert_eq!(get_bit_by_index(num, 0), false); // Least significant bit
        assert_eq!(get_bit_by_index(num, 1), false);
        assert_eq!(get_bit_by_index(num, 2), true);
        assert_eq!(get_bit_by_index(num, 3), true);
        assert_eq!(get_bit_by_index(num, 4), false);
        assert_eq!(get_bit_by_index(num, 5), true);
        assert_eq!(get_bit_by_index(num, 6), false);
        assert_eq!(get_bit_by_index(num, 7), true); // Most significant bit
    }

    #[test]
    fn test_get_bit_by_index_out_of_bounds() {
        let num: u8 = 0b10101100;

        // Test out-of-bounds index (8 or greater)
        assert_eq!(get_bit_by_index(num, 8), false);
        assert_eq!(get_bit_by_index(num, 9), false);
        assert_eq!(get_bit_by_index(num, 100), false);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds for u8")]
    fn test_get_bit_by_index_panic() {
        // Example with a panic version (if implemented)
        fn get_bit_by_index_panic(num: u8, index: usize) -> bool {
            assert!(index < 8, "Index out of bounds for u8");
            (num >> index) & 1 == 1
        }

        // This should panic with the message "Index out of bounds for u8"
        get_bit_by_index_panic(0b10101100, 8);
    }
}
