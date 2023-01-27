#include <iostream>
#include <thread>
#include <chrono>

int main() {
    char ch;
    std::cin >> ch;
    std::this_thread::sleep_for(std::chrono::milliseconds((ch - '0') * 600));
    std::cout << "Hello World!" << std::endl << ch << std::endl;
    return 0;
}
