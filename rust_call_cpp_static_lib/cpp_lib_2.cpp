/* 试一下一个cpp文件能不能被Rust导入
 * 但是业界，例如protobuf的cpp库都是一个cpp文件结合一个.h文件的，没有像我这样裸露一个.cpp文件作为库的
 * */
#include <algorithm>
extern "C" {
    void cpp_sort(int nums[], size_t n) {
        std::sort(nums, nums + n);
    }
}