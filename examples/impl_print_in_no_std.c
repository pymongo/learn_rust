#include <unistd.h> // write
#include <string.h> // strlen
#include <assert.h> // assert

int main() {
    const char* text = "Hello World\n";
    ssize_t write_len = write(1, text, strlen(text));
    assert(write_len == strlen(text));
    return 0;
}
