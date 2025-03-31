#include <gtest/gtest.h>
#include <add.hpp>

TEST(ADD, add) {
    EXPECT_TRUE(Add(1, 2) == 3);
}
