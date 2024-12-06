use crate::message_parser::messages::LZMessageName::{BroadcastAllLocoOff, BroadcastAllOff, BroadcastAllOn, BroadcastFeedback, BroadcastFeedbackExtended, BroadcastProgrammingMode, CommandNotFound, ExtendedVersionInformation, LZBusy, LZErrorsV30, LocoFreeV15, LocoFreeV23, LocoFunctionStateF0F12V30, LocoFunctionStateF13F28V36, LocoFunctionStateF29F68V40, LocoFunctionStateUpperUpperV40, LocoFunctionStateUpperV36, LocoInUseV30, LocoInfoDoubleLocos, LocoInfoMultipleLocos, LocoInfoMultipleLocosBase, LocoInfoNormalV30, LocoInfoSearchV30, LocoOccupiedV15, LocoOccupiedV23, ModelTime, NormalRequest, PoMEventReport, RequestAcknowledge, ServiceValueReport, SoftwareVersionReport23, SoftwareVersionReport30, StateLZ, SwitchInfo, SwitchInfoExtended, TransmissionError, TBD};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum LZMessageName {
    //Name of the message. Commend corresponds to the XpressNet Documentation entry
    NormalRequest,             //2.1 Normale Anfrage
    RequestAcknowledge,        //2.2 Quittierung
    TBD,                       //2.3 TBD
    BroadcastAllOn,            //2.4.1 BC „Alles An“
    BroadcastAllOff,           //2.4.2 BC „Alles Aus“ (Notaus)
    BroadcastAllLocoOff,       //2.4.3 BC „Alle Loks Aus“ (Nothalt)
    BroadcastProgrammingMode,  // 2.4.4 BC „Programmiermode“
    BroadcastFeedback,         //2.4.5 BC „Rückmeldung“
    BroadcastFeedbackExtended, //2.4.6 BC „Rückmeldung ab Weiche 1025“
    ServiceValueReport,
    SoftwareVersionReport23,
    SoftwareVersionReport30,
    StateLZ,
    ExtendedVersionInformation,
    PoMEventReport,
    ModelTime,
    TransmissionError,
    LZBusy,
    CommandNotFound,
    SwitchInfo,
    SwitchInfoExtended,
    LocoFreeV15,
    LocoOccupiedV15,
    LocoFreeV23,
    LocoOccupiedV23,
    LocoInfoNormalV30,
    LocoFunctionStateUpperV36,
    LocoFunctionStateUpperUpperV40,
    LocoInfoMultipleLocos,
    LocoInfoMultipleLocosBase,
    LocoInfoDoubleLocos,
    LocoInUseV30,
    LocoFunctionStateF0F12V30,
    LocoFunctionStateF13F28V36,
    LocoFunctionStateF29F68V40,
    LocoInfoSearchV30,
    DoubleLocoFreeV15,
    DoubleLocoUsedV15,
    DoubleLocoFreeV23,
    DoubleLocoUsedV23,
    DoubleLocoErrorV23,
    LZErrorsV30,
}
// TODO: Merge Definition and Builder. No reason these should be two different structs
pub struct LZMessageDefinition {
    pub name: LZMessageName,
    pub call_byte_value: u8,
    pub check_parity: bool,
    pub is_broadcast: bool,
    pub has_header: bool,
    pub header_byte_value: u8,
    pub data_byte_count: i8,
    pub has_identifier: bool,
    pub identifier_value: u8,
    pub identifier_match_partial: bool,
    pub min_version: f32,
    pub max_version: f32,
    pub has_xor: bool,
}
impl LZMessageDefinition {
    /// Creates a new `LZMessageBuilder` to configure an `LZMessageDefinition`.
    pub fn builder(name: LZMessageName, call_byte_value: u8) -> LZMessageBuilder {
        LZMessageBuilder::new(name, call_byte_value)
    }
}
pub struct LZMessageBuilder {
    name: LZMessageName,
    call_byte_value: u8,
    check_parity: bool,
    is_broadcast: bool,
    has_header: bool,
    header_byte_value: u8,
    data_byte_count: i8,
    has_identifier: bool,
    identifier_value: u8,
    identifier_match_partial: bool,
    min_version: f32,
    max_version: f32,
    has_xor: bool,
}
impl LZMessageBuilder {
    /// Creates a new builder with required fields.
    pub fn new(name: LZMessageName, call_byte_value: u8) -> Self {
        Self {
            name,
            call_byte_value,
            check_parity: false,
            is_broadcast: false,
            has_header: false,
            header_byte_value: 0,
            data_byte_count: 0,
            has_identifier: false,
            identifier_value: 0,
            identifier_match_partial: false,
            min_version: 0.0,
            max_version: f32::MAX,
            has_xor: false,
        }
    }

    pub fn check_parity(mut self) -> Self {
        self.check_parity = true;
        self
    }

    pub fn is_broadcast(mut self) -> Self {
        self.is_broadcast = true;
        self
    }

    pub fn header_byte_value(mut self, value: u8) -> Self {
        self.header_byte_value = value;
        self.has_header = true;
        self
    }

    pub fn data_byte_count(mut self, value: i8) -> Self {
        self.data_byte_count = value;
        self
    }

    pub fn identifier_value(mut self, value: u8) -> Self {
        self.identifier_value = value;
        self.has_identifier = true;
        self
    }
    pub fn identifier_partial_match(mut self) -> Self {
        self.identifier_match_partial = true;
        self
    }

    pub fn min_version(mut self, value: f32) -> Self {
        self.min_version = value;
        self
    }

    pub fn max_version(mut self, value: f32) -> Self {
        self.max_version = value;
        self
    }

    pub fn has_xor(mut self) -> Self {
        self.has_xor = true;
        self
    }

    /// Finalizes the builder and returns the constructed `LZMessageDefinition`.
    pub fn build(self) -> LZMessageDefinition {
        LZMessageDefinition {
            name: self.name,
            call_byte_value: self.call_byte_value,
            check_parity: self.check_parity,
            is_broadcast: self.is_broadcast,
            has_header: self.has_header,
            header_byte_value: self.header_byte_value,
            data_byte_count: self.data_byte_count,
            has_identifier: self.has_identifier,
            identifier_value: self.identifier_value,
            identifier_match_partial: self.identifier_match_partial,
            min_version: self.min_version,
            max_version: self.max_version,
            has_xor: self.has_xor,
        }
    }
}
pub struct LZMessage {
    pub definition: LZMessageDefinition,
    pub data: Vec<u8>,
}
pub fn get_definitions() -> Vec<LZMessageDefinition> {
    let mut definitions: Vec<LZMessageDefinition> = Vec::new();

    definitions.push(
        LZMessageDefinition::builder(NormalRequest, 0b10)
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(RequestAcknowledge, 0b00)
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(TBD, 0b01)
            .check_parity()
            .build(),
    );

    definitions.push(
        LZMessageDefinition::builder(BroadcastAllOn, 0b11)
            .is_broadcast()
            .header_byte_value(0b01100001)
            .data_byte_count(1)
            .has_xor()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(BroadcastAllOff, 0b11)
            .is_broadcast()
            .header_byte_value(0b01100001)
            .data_byte_count(1)
            .has_xor()
            .build(),
    );

    definitions.push(
        LZMessageDefinition::builder(BroadcastAllLocoOff, 0b11)
            .is_broadcast()
            .header_byte_value(0b10000001)
            .data_byte_count(2)
            .has_xor()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(BroadcastProgrammingMode, 0b11)
            .is_broadcast()
            .header_byte_value(0b01100001)
            .data_byte_count(2)
            .has_xor()
            .build(),
    );

    definitions.push(
        LZMessageDefinition::builder(BroadcastFeedback, 0b01)
            .check_parity()
            .is_broadcast()
            .header_byte_value(0b01000000)
            .data_byte_count(-1)
            .has_xor()
            .build(),
    );

    definitions.push(
        LZMessageDefinition::builder(BroadcastFeedbackExtended, 0b01)
            .is_broadcast()
            .header_byte_value(0b01000011)
            .data_byte_count(3)
            .has_xor()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(ServiceValueReport, 0b11)
            .check_parity()
            .header_byte_value(0b01100011)
            .data_byte_count(3)
            .has_xor()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(SoftwareVersionReport23, 0b11)
            .check_parity()
            .header_byte_value(0b01100010)
            .data_byte_count(2)
            .max_version(2.3)
            .has_xor()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(SoftwareVersionReport30, 0b11)
            .check_parity()
            .header_byte_value(0b01100011)
            .data_byte_count(3)
            .min_version(3.0)
            .has_xor()
            .build(),
    );

    definitions.push(
        LZMessageDefinition::builder(StateLZ, 0b11)
            .check_parity()
            .header_byte_value(0b01100010)
            .data_byte_count(2)
            .has_xor()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(ExtendedVersionInformation, 0b11)
            .check_parity()
            .header_byte_value(0b01100111)
            .data_byte_count(7)
            .has_xor()
            .build(),
    );

    definitions.push(
        LZMessageDefinition::builder(PoMEventReport, 0b11)
            .header_byte_value(0b01100100)
            .data_byte_count(4)
            .has_xor()
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(ModelTime, 0b11)
            .header_byte_value(0b01100100)
            .data_byte_count(4)
            .has_xor()
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(TransmissionError, 0b11)
            .header_byte_value(0b01100001)
            .data_byte_count(1)
            .has_xor()
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(LZBusy, 0b11)
            .header_byte_value(0b01100001)
            .data_byte_count(1)
            .has_xor()
            .check_parity()
            .build(),
    );

    definitions.push(
        LZMessageDefinition::builder(CommandNotFound, 0b11)
            .header_byte_value(0b01100001)
            .data_byte_count(1)
            .has_xor()
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(SwitchInfo, 0b11)
            .header_byte_value(0b01000010)
            .data_byte_count(2)
            .has_xor()
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(SwitchInfoExtended, 0b11)
            .header_byte_value(0b01000011)
            .data_byte_count(3)
            .has_xor()
            .check_parity()
            .build(),
    );

    definitions.push(
        LZMessageDefinition::builder(LocoFreeV15, 0b11)
            .header_byte_value(0b10000011)
            .data_byte_count(3)
            .max_version(1.5)
            .has_xor()
            .check_parity()
            .build(),
    );

    definitions.push(
        LZMessageDefinition::builder(LocoOccupiedV15, 0b11)
            .header_byte_value(0b10100011)
            .data_byte_count(3)
            .has_xor()
            .max_version(1.5)
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(LocoFreeV23, 0b11)
            .header_byte_value(0b10000100)
            .data_byte_count(4)
            .has_xor()
            .min_version(2.0)
            .max_version(2.3)
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(LocoOccupiedV23, 0b11)
            .header_byte_value(0b10100100)
            .data_byte_count(4)
            .has_xor()
            .min_version(2.0)
            .max_version(2.3)
            .check_parity()
            .build(),
    );

    definitions.push(
        LZMessageDefinition::builder(LocoInfoNormalV30, 0b11)
            .header_byte_value(0b11100100)
            .data_byte_count(3)
            .has_xor()
            .min_version(3.0)
            .identifier_value(0b00000000)
            .identifier_partial_match()
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(LocoFunctionStateUpperV36, 0b11)
            .header_byte_value(0b11100011)
            .data_byte_count(2)
            .has_xor()
            .min_version(3.6)
            .identifier_value(0b01010010)
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(LocoFunctionStateUpperUpperV40, 0b11)
            .header_byte_value(0b11100110)
            .data_byte_count(5)
            .has_xor()
            .min_version(4.0)
            .identifier_value(0b01010011)
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(LocoInfoMultipleLocos, 0b11)
            .header_byte_value(0b11100101)
            .data_byte_count(4)
            .has_xor()
            .identifier_value(0b00010000)
            .identifier_partial_match()
            .min_version(3.0)
            .check_parity()
            .build(),
    );

    definitions.push(
        LZMessageDefinition::builder(LocoInfoMultipleLocosBase, 0b11)
            .header_byte_value(0b11100010)
            .data_byte_count(1)
            .has_xor()
            .identifier_value(0b00100000)
            .identifier_partial_match()
            .min_version(3.0)
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(LocoInfoDoubleLocos, 0b11)
            .header_byte_value(0b11100110)
            .data_byte_count(5)
            .has_xor()
            .identifier_value(0b01100000)
            .identifier_partial_match()
            .min_version(3.0)
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(LocoInUseV30, 0b11)
            .header_byte_value(0b11100011)
            .data_byte_count(2)
            .has_xor()
            .identifier_value(0b01000000)
            .min_version(3.0)
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(LocoFunctionStateF0F12V30, 0b11)
            .header_byte_value(0b11100011)
            .data_byte_count(2)
            .has_xor()
            .identifier_value(0b01010000)
            .min_version(3.0)
            .check_parity()
            .build(),
    );

    definitions.push(
        LZMessageDefinition::builder(LocoFunctionStateF13F28V36, 0b11)
            .header_byte_value(0b11100100)
            .data_byte_count(3)
            .has_xor()
            .identifier_value(0b01010001)
            .min_version(3.6)
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(LocoFunctionStateF29F68V40, 0b11)
            .header_byte_value(0b11100110)
            .data_byte_count(5)
            .has_xor()
            .identifier_value(0b01010100)
            .min_version(4.0)
            .check_parity()
            .build(),
    );
    definitions.push(
        LZMessageDefinition::builder(LocoInfoSearchV30, 0b11)
            .header_byte_value(0b11100011)
            .data_byte_count(2)
            .has_xor()
            .identifier_value(0b00110000)
            .identifier_partial_match()
            .min_version(3.0)
            .check_parity()
            .build(),
    );


    // TODO: implement stuff for DTR and MTR here

    definitions.push(
        LZMessageDefinition::builder(LZErrorsV30, 0b11)
            .header_byte_value(0b11100001)
            .data_byte_count(0)
            .has_xor()
            .identifier_value(0b10000000)
            .identifier_partial_match()
            .min_version(3.0)
            .check_parity()
            .build(),
    );

    definitions
}

// Filter comes here now
pub struct LZMessageDefinitionFilter {
    pub possible_definitions: Vec<LZMessageDefinition>
}

impl LZMessageDefinitionFilter {
    pub fn new() -> LZMessageDefinitionFilter {
        LZMessageDefinitionFilter {
            possible_definitions: get_definitions()
        }
    }

    pub fn filter_call(&mut self, call_byte: u8) {
        let call_address = call_byte & 0b00011111;

        // If call is a broadcast, remove all non broadcasts from the list
        if call_address == 0 {
            self.possible_definitions.retain(|msg| msg.is_broadcast)

        }



    }
}