#include "cpp_lib.h"
void cpp_sort(int nums[], size_t n) {
    // end指针的实际偏移量是 start+n*sizeof(int)
    std::sort(nums, nums + n);
}