#include "messages.h"

void LZMessageDefinitionFilter::filter_call(uint8_t call_byte) {
    // If parity check fails, discard all
    if (check_parity(call_byte)) {
        this->possible_definitions.clear();
        return;
    }
    this->call_address = call_byte & 0b00011111;

    // Check if it is broadcast or not
    if (call_address == 0) {
        this->possible_definitions.erase(
            std::remove_if(this->possible_definitions.begin(), this->possible_definitions.end(),
                           [](const LZMessageDefinition &msg) { return !msg.is_broadcast; }),
            this->possible_definitions.end());
    } else {
        this->possible_definitions.erase(
            std::remove_if(this->possible_definitions.begin(), this->possible_definitions.end(),
                           [](const LZMessageDefinition &msg) { return msg.is_broadcast; }),
            this->possible_definitions.end());
    }

    // Filter by call ID
    uint8_t call_value = (call_byte & 0b01100000) >> 5;
    this->possible_definitions.erase(
        std::remove_if(this->possible_definitions.begin(), this->possible_definitions.end(),
                       [call_value](const LZMessageDefinition &msg) {
                           return msg.call_byte_value != call_value;
                       }),
        this->possible_definitions.end());
}

void LZMessageDefinitionFilter::filter_header(uint8_t header_byte, uint8_t call_byte) {
    // Check if the call is feedback regarding switches
    if (this->possible_definitions.size() == 2 &&
        std::ranges::find_if(this->possible_definitions,
                             [](const LZMessageDefinition &def) {
                                 return def.name == LZMessageName::BroadcastFeedback || def.name ==
                                        LZMessageName::BroadcastFeedbackExtended;
                             }) != this->possible_definitions.end()) {
        // Check if the header is correct
        if ((header_byte >> 4) == 0b0100) {
            uint8_t data_length = header_byte & 0b00001111;
            if (data_length == 3) {
                this->possible_definitions.erase(
                    std::ranges::remove_if(this->possible_definitions,
                                           [](const LZMessageDefinition &msg) {
                                               return msg.name != LZMessageName::BroadcastFeedbackExtended;
                                           }).begin(),
                    this->possible_definitions.end());
                return;
            }
            if (data_length % 2 == 0) {
                this->possible_definitions.erase(
                    std::ranges::remove_if(this->possible_definitions,
                                           [](const LZMessageDefinition &msg) {
                                               return msg.name != LZMessageName::BroadcastFeedback;
                                           }).begin(),
                    this->possible_definitions.end());
                return;
            }
            this->possible_definitions.clear();
            return;
        }
        this->possible_definitions.clear();
        printf("Invalid header byte for broadcastfeedback");
        return;
    }
}

void LZMessageDefinitionFilter::reset() {
    possible_definitions = get_definitions();
    this->call_address = 255;
}

bool LZMessageDefinitionFilter::check_parity(uint8_t data) {
    int count = 0;
    for (int i = 0; i < 8; i++) {
        if (data & (1 << i)) {
            count++;
        }
    }
    return count % 2 == 0;
}

std::vector<LZMessageDefinition> LZMessageDefinitionFilter::remaining_message_definitions() {
    return this->possible_definitions;
}

uint8_t LZMessageDefinitionFilter::get_call_address() const {
    return this->call_address;
}
