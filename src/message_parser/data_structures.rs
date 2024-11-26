struct Locomotive {
    id: u16,
}

pub struct RailwaySetupState {
    id: u32,
    emergency_off: bool,
    locos: Vec<Locomotive>,
    //TODO: Add the rest here
}

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

pub struct LZMessageDefinition {
    pub name: LZMessageName,
    pub call_byte_template: String,
    pub has_header: bool,
    pub header_byte_template: String,
    pub data_byte_count: i8,
    pub has_identifier: bool,
    pub identifier_template: String,
    pub min_version: f32,
    pub max_version: f32,
    pub has_xor: bool,
}

pub fn get_definitions() -> Vec<LZMessageDefinition> {
    let mut definitions: Vec<LZMessageDefinition> = Vec::new();

    definitions.push(LZMessageDefinition {
        name: LZMessageName::NormalRequest,
        call_byte_template: "P10AAAAA".to_string(),
        has_header: false,
        header_byte_template: "".to_string(),
        data_byte_count: 0,
        has_identifier: false,
        identifier_template: "".to_string(),
        min_version: 0.0,
        max_version: 99.0,
        has_xor: false,
    });
    definitions.push(LZMessageDefinition {
        name: LZMessageName::RequestAcknowledge,
        call_byte_template: "P00AAAAA".to_string(),
        has_header: false,
        header_byte_template: "".to_string(),
        data_byte_count: 0,
        has_identifier: false,
        identifier_template: "".to_string(),
        min_version: 0.0,
        max_version: 99.0,
        has_xor: false,
    });
    definitions.push(LZMessageDefinition {
        name: LZMessageName::TBD,
        call_byte_template: "P01AAAAA".to_string(),
        has_header: false,
        header_byte_template: "".to_string(),
        data_byte_count: 0,
        has_identifier: false,
        identifier_template: "".to_string(),
        min_version: 0.0,
        max_version: 99.0,
        has_xor: false,
    });
    definitions.push(LZMessageDefinition {
        name: LZMessageName::BroadcastAllOn,
        call_byte_template: "01100000".to_string(),
        has_header: true,
        header_byte_template: "01100001".to_string(),
        data_byte_count: 0,
        has_identifier: false,
        identifier_template: "".to_string(),
        min_version: 0.0,
        max_version: 99.0,
        has_xor: true,
    });
    definitions.push(LZMessageDefinition {
        name: LZMessageName::BroadcastAllOff,
        call_byte_template: "01100000".to_string(),
        has_header: true,
        header_byte_template: "01100001".to_string(),
        data_byte_count: 1,
        has_identifier: false,
        identifier_template: "".to_string(),
        min_version: 0.0,
        max_version: 99.0,
        has_xor: true,
    });
    definitions.push(LZMessageDefinition {
        name: LZMessageName::BroadcastAllLocoOff,
        call_byte_template: "01100000".to_string(),
        has_header: true,
        header_byte_template: "10000001".to_string(),
        data_byte_count: 1,
        has_identifier: false,
        identifier_template: "".to_string(),
        min_version: 0.0,
        max_version: 99.0,
        has_xor: true,
    });
    definitions.push(LZMessageDefinition {
        name: LZMessageName::BroadcastProgrammingMode,
        call_byte_template: "01100000".to_string(),
        has_header: true,
        header_byte_template: "01100001".to_string(),
        data_byte_count: 1,
        has_identifier: false,
        identifier_template: "".to_string(),
        min_version: 0.0,
        max_version: 99.0,
        has_xor: true,
    });
    definitions.push(LZMessageDefinition {
        name: LZMessageName::BroadcastFeedback,
        call_byte_template: "P0100000".to_string(),
        has_header: true,
        header_byte_template: "0100NNNN".to_string(),
        data_byte_count: -1,
        has_identifier: false,
        identifier_template: "".to_string(),
        min_version: 0.0,
        max_version: 99.0,
        has_xor: true,
    });
    definitions.push(LZMessageDefinition {
        name: LZMessageName::BroadcastFeedbackExtended,
        call_byte_template: "01100000".to_string(),
        has_header: true,
        header_byte_template: "01000011".to_string(),
        data_byte_count: 3,
        has_identifier: false,
        identifier_template: "".to_string(),
        min_version: 0.0,
        max_version: 99.0,
        has_xor: true,
    });
    definitions.push(LZMessageDefinition {
        name: LZMessageName::ServiceValueReport,
        call_byte_template: "P11AAAAA".to_string(),
        has_header: true,
        header_byte_template: "01100011".to_string(),
        data_byte_count: 2,
        has_identifier: true,
        identifier_template: "00100000".to_string(),
        min_version: 0.0,
        max_version: 99.0,
        has_xor: true,
    });

    definitions.push(LZMessageDefinition {
        name: LZMessageName::SoftwareVersionReport23,
        call_byte_template: "P11AAAAA".to_string(),
        has_header: true,
        header_byte_template: "01100010".to_string(),
        data_byte_count: 2,
        has_identifier: false,
        identifier_template: "".to_string(),
        min_version: 0.0,
        max_version: 2.3,
        has_xor: true,
    });
    definitions.push(LZMessageDefinition {
        name: LZMessageName::SoftwareVersionReport30,
        call_byte_template: "P11AAAAA".to_string(),
        has_header: true,
        header_byte_template: "01100011".to_string(),
        data_byte_count: 3,
        has_identifier: false,
        identifier_template: "".to_string(),
        min_version: 0.0,
        max_version: 2.3,
        has_xor: true,
    });
    definitions.push(LZMessageDefinition {
        name: LZMessageName::StateLZ,
        call_byte_template: "P11AAAAA".to_string(),
        has_header: true,
        header_byte_template: "01100010".to_string(),
        data_byte_count: 2,
        has_identifier: false,
        identifier_template: "".to_string(),
        min_version: 0.0,
        max_version: 99.0,
        has_xor: true,
    });

    definitions
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::PartialEq;

    #[test]
    fn check_duplicates() {
        let definitions = get_definitions();
        for definition in definitions {
            let definitions2 = get_definitions();
            for definition2 in definitions2 {
                if (definition.header_byte_template == definition2.header_byte_template
                    && definition.call_byte_template == definition2.call_byte_template
                    && definition.name != definition2.name)
                {
                    println!("Double found");
                }
            }
        }
    }
}
