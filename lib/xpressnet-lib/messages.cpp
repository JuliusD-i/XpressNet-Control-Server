#include "messages.h"

void LZMessageDefinitionFilter::filter_call(uint8_t call_byte) {

    if (check_parity(call_byte)) {
        this->possible_definitions.clear();
        return;
    }

    this->call_address = call_byte & 0b00011111;

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

    uint8_t call_value = (call_byte & 0b01100000) >> 5;
    this->possible_definitions.erase(
        std::remove_if(this->possible_definitions.begin(), this->possible_definitions.end(),
                       [call_value](const LZMessageDefinition &msg) {
                           return msg.call_byte_value != call_value;
                       }),
        this->possible_definitions.end());

    printf("Filtered definitions: %d\n", possible_definitions.size());
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
