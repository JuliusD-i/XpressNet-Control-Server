//
// Created by julius on 23.12.24.
//

#ifndef MESSAGES_H
#define MESSAGES_H
#include <algorithm>
#include <cstdint>
#include <cfloat>
#include <vector>

#include <iostream>
#include <vector>
#include <string>
#include <cstdint>
#include <algorithm>
#include <cassert>

enum class LZMessageName {
    NormalRequest,
    RequestAcknowledge,
    TBD,
    BroadcastAllOn,
    BroadcastAllOff,
    BroadcastAllLocoOff,
    BroadcastProgrammingMode,
    BroadcastFeedback,
    BroadcastFeedbackExtended,
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
    LZErrorsV30
};

struct LZMessageDefinition {
    LZMessageName name;
    uint8_t call_byte_value;
    bool check_parity;
    bool is_broadcast;
    bool has_header;
    uint8_t header_byte_value;
    int8_t data_byte_count;
    bool has_identifier;
    uint8_t identifier_value;
    bool identifier_match_partial;
    float min_version;
    float max_version;
    bool has_xor;

    static LZMessageDefinition builder(LZMessageName name, uint8_t call_byte_value);
};

struct LZMessageBuilder {
    LZMessageName name;
    uint8_t call_byte_value;
    bool check_parity;
    bool is_broadcast;
    bool has_header;
    uint8_t header_byte_value;
    int8_t data_byte_count;
    bool has_identifier;
    uint8_t identifier_value;
    bool identifier_match_partial;
    float min_version;
    float max_version;
    bool has_xor;

    LZMessageBuilder(LZMessageName name, uint8_t call_byte_value)
        : name(name), call_byte_value(call_byte_value), check_parity(false), is_broadcast(false),
          has_header(false), header_byte_value(0), data_byte_count(0), has_identifier(false),
          identifier_value(0), identifier_match_partial(false), min_version(0.0), max_version(FLT_MAX),
          has_xor(false) {
    }

    LZMessageBuilder &set_check_parity() {
        check_parity = true;
        return *this;
    }

    LZMessageBuilder &set_is_broadcast() {
        is_broadcast = true;
        return *this;
    }

    LZMessageBuilder &set_header_byte_value(uint8_t value) {
        header_byte_value = value;
        has_header = true;
        return *this;
    }

    LZMessageBuilder &set_data_byte_count(int8_t value) {
        data_byte_count = value;
        return *this;
    }

    LZMessageBuilder &set_identifier_value(uint8_t value) {
        identifier_value = value;
        has_identifier = true;
        return *this;
    }

    LZMessageBuilder &set_identifier_partial_match() {
        identifier_match_partial = true;
        return *this;
    }

    LZMessageBuilder &set_min_version(float value) {
        min_version = value;
        return *this;
    }

    LZMessageBuilder &set_max_version(float value) {
        max_version = value;
        return *this;
    }

    LZMessageBuilder &set_has_xor() {
        has_xor = true;
        return *this;
    }

    LZMessageDefinition build() {
        return LZMessageDefinition{
            name, call_byte_value, check_parity, is_broadcast, has_header,
            header_byte_value, data_byte_count, has_identifier, identifier_value,
            identifier_match_partial, min_version, max_version, has_xor
        };
    }
};

inline LZMessageDefinition LZMessageDefinition::builder(LZMessageName name, uint8_t call_byte_value) {
    return LZMessageBuilder(name, call_byte_value).build();
}

inline std::vector<LZMessageDefinition> get_definitions() {
    std::vector<LZMessageDefinition> definitions;

    definitions.push_back(LZMessageBuilder(LZMessageName::NormalRequest, 0b10).set_check_parity().build());
    definitions.push_back(LZMessageBuilder(LZMessageName::RequestAcknowledge, 0b00).set_check_parity().build());
    definitions.push_back(LZMessageBuilder(LZMessageName::TBD, 0b01).set_check_parity().build());

    definitions.push_back(LZMessageBuilder(LZMessageName::BroadcastAllOn, 0b11)
        .set_is_broadcast()
        .set_header_byte_value(0b01100001)
        .set_data_byte_count(1)
        .set_has_xor()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::BroadcastAllOff, 0b11)
        .set_is_broadcast()
        .set_header_byte_value(0b01100001)
        .set_data_byte_count(1)
        .set_has_xor()
        .build());

    definitions.push_back(LZMessageBuilder(LZMessageName::BroadcastAllLocoOff, 0b11)
        .set_is_broadcast()
        .set_header_byte_value(0b10000001)
        .set_data_byte_count(2)
        .set_has_xor()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::BroadcastProgrammingMode, 0b11)
        .set_is_broadcast()
        .set_header_byte_value(0b01100001)
        .set_data_byte_count(2)
        .set_has_xor()
        .build());

    definitions.push_back(LZMessageBuilder(LZMessageName::BroadcastFeedback, 0b01)
        .set_check_parity()
        .set_is_broadcast()
        .set_header_byte_value(0b01000000)
        .set_data_byte_count(-1)
        .set_has_xor()
        .build());

    definitions.push_back(LZMessageBuilder(LZMessageName::BroadcastFeedbackExtended, 0b01)
        .set_is_broadcast()
        .set_header_byte_value(0b01000011)
        .set_data_byte_count(3)
        .set_has_xor()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::ServiceValueReport, 0b11)
        .set_check_parity()
        .set_header_byte_value(0b01100011)
        .set_data_byte_count(3)
        .set_has_xor()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::SoftwareVersionReport23, 0b11)
        .set_check_parity()
        .set_header_byte_value(0b01100010)
        .set_data_byte_count(2)
        .set_max_version(2.3)
        .set_has_xor()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::SoftwareVersionReport30, 0b11)
        .set_check_parity()
        .set_header_byte_value(0b01100011)
        .set_data_byte_count(3)
        .set_min_version(3.0)
        .set_has_xor()
        .build());

    definitions.push_back(LZMessageBuilder(LZMessageName::StateLZ, 0b11)
        .set_check_parity()
        .set_header_byte_value(0b01100010)
        .set_data_byte_count(2)
        .set_has_xor()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::ExtendedVersionInformation, 0b11)
        .set_check_parity()
        .set_header_byte_value(0b01100111)
        .set_data_byte_count(7)
        .set_has_xor()
        .build());

    definitions.push_back(LZMessageBuilder(LZMessageName::PoMEventReport, 0b11)
        .set_header_byte_value(0b01100100)
        .set_data_byte_count(4)
        .set_has_xor()
        .set_check_parity()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::ModelTime, 0b11)
        .set_header_byte_value(0b01100100)
        .set_data_byte_count(4)
        .set_has_xor()
        .set_check_parity()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::TransmissionError, 0b11)
        .set_header_byte_value(0b01100001)
        .set_data_byte_count(1)
        .set_has_xor()
        .set_check_parity()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::LZBusy, 0b11)
        .set_header_byte_value(0b01100001)
        .set_data_byte_count(1)
        .set_has_xor()
        .set_check_parity()
        .build());

    definitions.push_back(LZMessageBuilder(LZMessageName::CommandNotFound, 0b11)
        .set_header_byte_value(0b01100001)
        .set_data_byte_count(1)
        .set_has_xor()
        .set_check_parity()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::SwitchInfo, 0b11)
        .set_header_byte_value(0b01000010)
        .set_data_byte_count(2)
        .set_has_xor()
        .set_check_parity()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::SwitchInfoExtended, 0b11)
        .set_header_byte_value(0b01000011)
        .set_data_byte_count(3)
        .set_has_xor()
        .set_check_parity()
        .build());

    definitions.push_back(LZMessageBuilder(LZMessageName::LocoFreeV15, 0b11)
        .set_header_byte_value(0b10000011)
        .set_data_byte_count(3)
        .set_max_version(1.5)
        .set_has_xor()
        .set_check_parity()
        .build());

    definitions.push_back(LZMessageBuilder(LZMessageName::LocoOccupiedV15, 0b11)
        .set_header_byte_value(0b10100011)
        .set_data_byte_count(3)
        .set_has_xor()
        .set_max_version(1.5)
        .set_check_parity()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::LocoFreeV23, 0b11)
        .set_header_byte_value(0b10000100)
        .set_data_byte_count(4)
        .set_has_xor()
        .set_min_version(2.0)
        .set_max_version(2.3)
        .set_check_parity()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::LocoOccupiedV23, 0b11)
        .set_header_byte_value(0b10100100)
        .set_data_byte_count(4)
        .set_has_xor()
        .set_min_version(2.0)
        .set_max_version(2.3)
        .set_check_parity()
        .build());

    definitions.push_back(LZMessageBuilder(LZMessageName::LocoInfoNormalV30, 0b11)
        .set_header_byte_value(0b11100100)
        .set_data_byte_count(3)
        .set_has_xor()
        .set_min_version(3.0)
        .set_identifier_value(0b00000000)
        .set_identifier_partial_match()
        .set_check_parity()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::LocoFunctionStateUpperV36, 0b11)
        .set_header_byte_value(0b11100011)
        .set_data_byte_count(2)
        .set_has_xor()
        .set_min_version(3.6)
        .set_identifier_value(0b01010010)
        .set_check_parity()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::LocoFunctionStateUpperUpperV40, 0b11)
        .set_header_byte_value(0b11100110)
        .set_data_byte_count(5)
        .set_has_xor()
        .set_min_version(4.0)
        .set_identifier_value(0b01010011)
        .set_check_parity()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::LocoInfoMultipleLocos, 0b11)
        .set_header_byte_value(0b11100101)
        .set_data_byte_count(4)
        .set_has_xor()
        .set_identifier_value(0b00010000)
        .set_identifier_partial_match()
        .set_min_version(3.0)
        .set_check_parity()
        .build());

    definitions.push_back(LZMessageBuilder(LZMessageName::LocoInfoMultipleLocosBase, 0b11)
        .set_header_byte_value(0b11100010)
        .set_data_byte_count(1)
        .set_has_xor()
        .set_identifier_value(0b00100000)
        .set_identifier_partial_match()
        .set_min_version(3.0)
        .set_check_parity()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::LocoInfoDoubleLocos, 0b11)
        .set_header_byte_value(0b11100110)
        .set_data_byte_count(5)
        .set_has_xor()
        .set_identifier_value(0b01100000)
        .set_identifier_partial_match()
        .set_min_version(3.0)
        .set_check_parity()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::LocoInUseV30, 0b11)
        .set_header_byte_value(0b11100011)
        .set_data_byte_count(2)
        .set_has_xor()
        .set_identifier_value(0b01000000)
        .set_min_version(3.0)
        .set_check_parity()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::LocoFunctionStateF0F12V30, 0b11)
        .set_header_byte_value(0b11100011)
        .set_data_byte_count(2)
        .set_has_xor()
        .set_identifier_value(0b01010000)
        .set_min_version(3.0)
        .set_check_parity()
        .build());

    definitions.push_back(LZMessageBuilder(LZMessageName::LocoFunctionStateF13F28V36, 0b11)
        .set_header_byte_value(0b11100100)
        .set_data_byte_count(3)
        .set_has_xor()
        .set_identifier_value(0b01010001)
        .set_min_version(3.6)
        .set_check_parity()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::LocoFunctionStateF29F68V40, 0b11)
        .set_header_byte_value(0b11100110)
        .set_data_byte_count(5)
        .set_has_xor()
        .set_identifier_value(0b01010100)
        .set_min_version(4.0)
        .set_check_parity()
        .build());
    definitions.push_back(LZMessageBuilder(LZMessageName::LocoInfoSearchV30, 0b11)
        .set_header_byte_value(0b11100011)
        .set_data_byte_count(2)
        .set_has_xor()
        .set_identifier_value(0b00110000)
        .set_identifier_partial_match()
        .set_min_version(3.0)
        .set_check_parity()
        .build());

    definitions.push_back(LZMessageBuilder(LZMessageName::LZErrorsV30, 0b11)
        .set_header_byte_value(0b11100001)
        .set_data_byte_count(0)
        .set_has_xor()
        .set_identifier_value(0b10000000)
        .set_identifier_partial_match()
        .set_min_version(3.0)
        .set_check_parity()
        .build());

    return definitions;
}

struct LZMessage {
    LZMessageDefinition definition;
    std::vector<uint8_t> data;
};


class LZMessageDefinitionFilter {
public:
    LZMessageDefinitionFilter() : possible_definitions(get_definitions()) {}

    std::vector<LZMessageDefinition> remaining_message_definitions();

    [[nodiscard]] uint8_t get_call_address() const;
    static bool check_parity(uint8_t data);
    void reset();
    void filter_call(uint8_t call_byte);
    void filter_header(uint8_t header_byte, uint8_t call_byte);



private:
    std::vector<LZMessageDefinition> possible_definitions;
    uint8_t call_address = 255;
};

#endif //MESSAGES_H
