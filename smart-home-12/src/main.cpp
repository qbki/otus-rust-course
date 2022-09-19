#include<iostream>
#include "./lib.h"

int main() {
    auto outlet = allocate_outlet("Some Outlet");

    std::cout << "*** Properties ***" << std::endl;
    std::cout << "Name: " << get_name(outlet) << std::endl;
    std::cout << "Power: " << get_power(outlet) << std::endl;
    std::cout << "Switch: " << static_cast<bool>(get_switch(outlet)) << std::endl;
    std::cout << "*** Report ***" << std::endl;
    std::cout << report(outlet) << std::endl;

    std::cout << (outlet == NULL) << std::endl;
    terminate_outlet(&outlet);
    std::cout << (outlet == NULL) << std::endl;
}
