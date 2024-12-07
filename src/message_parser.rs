use crate::message_parser::messages::{
    get_definitions, LZMessage, LZMessageDefinition, LZMessageDefinitionFilter,
};
use crate::message_parser::MessageParserState::WaitingForCall;
use crate::railway_state::RailwaySetupState;
use std::cell::RefCell;
use std::rc::Rc;

mod messages;

#[derive(Debug, Eq, PartialEq)]
pub enum MessageParserState {
    WaitingForCall,
    WaitingForHeader,
    WaitingForIdentifier,
    WaitingForData,
    ProcessingData,
}
pub struct MessageParser {
    state: MessageParserState,
    collected_data: Vec<u8>,
    message_filter: LZMessageDefinitionFilter,
    railway_setup_state: Rc<RefCell<RailwaySetupState>>,
}
impl MessageParser {
    pub fn new(state: Rc<RefCell<RailwaySetupState>>) -> MessageParser {
        MessageParser {
            state: MessageParserState::WaitingForCall,
            collected_data: Vec::new(),
            message_filter: LZMessageDefinitionFilter::new(),
            railway_setup_state: state,
        }
    }

    pub fn parse_data(
        mut self,
        data: u8,
        _receive_time: u64,
    ) -> Result<LZMessage, MessageParserState> {
        if self.state == WaitingForCall {
            self.collected_data.clear();
            self.collected_data.push(data);
            self.message_filter.filter_call(data);
            if self.message_filter.possible_definitions.len() == 1 {}
        }

        Result::Err(MessageParserState::WaitingForCall)
    }
}
