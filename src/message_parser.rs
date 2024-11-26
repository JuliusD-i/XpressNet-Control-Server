use crate::message_parser::data_structures::{get_definitions, LZMessageDefinition};

mod data_structures;

enum MessageParserState {
    WaitingForCall,
    WaitingForHeader,
    WaitingForIdentifier,
    WaitingForData,
    ProcessingData,
}
pub struct MessageParser {
    state: MessageParserState,
    collected_data: Vec<u8>,
    message_definitions: Vec<LZMessageDefinition>,
}
impl MessageParser {
    pub fn new() -> MessageParser {
        MessageParser {
            state:MessageParserState::WaitingForCall,
            collected_data: Vec::new(),
            message_definitions: get_definitions(),
        }
    }

    pub fn parse_call(_data: u8, _receive_time: u64) -> LZMessageDefinition {
        LZMessageDefinition {
            name: data_structures::LZMessageName::NormalRequest,
            call_byte_template: "P10AAAAA".to_string(),
            has_header: false,
            header_byte_template: "".to_string(),
            data_byte_count: 0,
            has_identifier: false,
            identifier_template: "".to_string(),
            min_version: 0.0,
            max_version: 99.0,
            has_xor: false,
        }
    }

}


