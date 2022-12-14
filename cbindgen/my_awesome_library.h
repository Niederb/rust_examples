#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct MyPoint {
  uint8_t x;
  uint32_t y;
  uint8_t z;
};

extern "C" {

MyPoint give_me_the_point();

int32_t square_all_the_things(int32_t i);

} // extern "C"
