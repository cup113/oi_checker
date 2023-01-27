#include <iostream>
#include <thread>
#include <chrono>

int main() {
    static char ch[3];
    std::cin >> ch;
    std::this_thread::sleep_for(std::chrono::milliseconds((ch[0] - '0') * 150));
    std::cout << "Hello World!" << std::endl << ch << std::endl;
    return 0;
}
