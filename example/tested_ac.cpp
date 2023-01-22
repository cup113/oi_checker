#include <iostream>

int main() {
    static char ch[3];
    std::cin >> ch;
    std::cout << "Hello World!" << std::endl << ch << std::endl;
    return 0;
}