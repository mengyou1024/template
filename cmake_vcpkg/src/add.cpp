#include "add.hpp"
{% if enable_rust -%}
#include <cxxbridge-cpp/foo/mod.h>
#include <cxxbridge-cpp/lib.h>
#include <vector>
{% endif %}
int Add(int a, int b) {
    return a + b;
}
{% if enable_rust %}
void HelloRust(void) {
    std::vector<uint64_t>              input = {4, 5, 6};
    rust::Slice<const ::std::uint64_t> slice{input.data(), input.size()};
    lib::print(slice);
    foo::print(slice);
}
{% endif %}