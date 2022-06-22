#pragma once
#include <univalue.h>

namespace rust::behavior {
template <typename Try, typename Fail>
void trycatch(Try &&func, Fail &&fail) noexcept try {
  func();
} catch (const std::exception &e) {
  fail(e.what());
} catch (const UniValue &e) {
  auto s = e.write();
  fail(s.c_str());
}
}
