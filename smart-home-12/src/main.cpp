#include<iostream>

using OutletHandle = void*;

extern "C" {
    OutletHandle allocate_outlet(const char*);
    void terminate_outlet(OutletHandle*);

    const char* report(OutletHandle);

    double get_power(OutletHandle);

    int get_switch(OutletHandle);
    void set_switch(OutletHandle, int);
}

int main() {
    auto outlet = allocate_outlet("Some Outlet");

    std::cout << "*** Properties ***" << std::endl;
    std::cout << "Power: " << get_power(outlet) << std::endl;
    std::cout << "Switch: " << static_cast<bool>(get_switch(outlet)) << std::endl;
    std::cout << "*** Report ***" << std::endl;
    std::cout << report(outlet) << std::endl;

    set_switch(outlet, 1);

    std::cout << "*** Modified Report ***" << std::endl;
    std::cout << report(outlet) << std::endl;

    std::cout << (outlet == NULL) << std::endl;
    terminate_outlet(&outlet);
    std::cout << (outlet == NULL) << std::endl;
}
