#include "main.hpp"
{% if enable_rust -%}
#include <cxxbridge-cpp/foo/mod.h>
#include <cxxbridge-cpp/lib.h>
{% endif -%}
#include <iostream>
{% if enable_rust -%}
#include <vector>
{% endif %}
int main() {
    {% if enable_rust -%}
    std::vector<uint64_t>              input = {4, 5, 6};
    rust::Slice<const ::std::uint64_t> slice{input.data(), input.size()};
    lib::print(slice);
    foo::print(slice);
    {% endif -%}
    std::cout << HELLO;
    return 0;
}
