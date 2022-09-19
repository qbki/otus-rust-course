#include<iostream>
#include "./lib.h"

int main() {
    auto outlet = allocate_outlet("Some Outlet");

    printf("Power %lf\n", get_power(outlet));
}
