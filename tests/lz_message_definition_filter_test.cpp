//
// Created by julius on 23.12.24.
//
#include <gtest/gtest.h>
#include "../lib/xpressnet-lib/messages.h"

class LZMessageDefinitionFilterTest : public ::testing::Test {
protected:
    LZMessageDefinitionFilter filter;
};

TEST_F(LZMessageDefinitionFilterTest, FiltersBroadcastMessages) {
    uint8_t calls[] = {0b00000000, 0b10100000, 0b01000000, 0b01100000};
    int expected_sizes[] = {0, 2, 0, 4};
    for (int i = 0; i < 4; i++) {
        filter.filter_call(calls[i]);
        EXPECT_EQ(filter.remaining_message_definitions().size(), expected_sizes[i]);
        auto remaining = filter.remaining_message_definitions();
        EXPECT_TRUE(std::all_of(remaining.begin(), remaining.end(),
            [](const LZMessageDefinition &msg) {
            return msg.is_broadcast;
            }));
        filter.reset();
    }
}


TEST_F(LZMessageDefinitionFilterTest, FilterRegularCall) {

    filter.filter_call(0b01000001);
    auto remaining = filter.remaining_message_definitions();
    EXPECT_EQ(remaining.size(), 1);
    EXPECT_EQ(remaining[0].name, LZMessageName::NormalRequest);
    filter.reset();


    filter.filter_call(0b10000001);
    remaining = filter.remaining_message_definitions();
    EXPECT_EQ(remaining.size(), 1);
    EXPECT_EQ(remaining[0].name, LZMessageName::RequestAcknowledge);
    filter.reset();


    filter.filter_call(0b00100001);
    remaining = filter.remaining_message_definitions();
    EXPECT_EQ(remaining.size(), 1);
    EXPECT_EQ(remaining[0].name, LZMessageName::TBD);
    filter.reset();
}
/*
TEST_F(LZMessageDefinitionFilterTest, FiltersByCallByteValue) {
    filter.filter_call(0b00100000);
    EXPECT_EQ(filter.remaining_message_definitions().size(), 1);
    EXPECT_EQ(filter.remaining_message_definitions()[0].call_byte_value, 0b00100000);
}

TEST_F(LZMessageDefinitionFilterTest, NoDefinitionsLeftAfterFiltering) {
    filter.filter_call(0b10000000);
    EXPECT_TRUE(filter.remaining_message_definitions().empty());
}
*/
