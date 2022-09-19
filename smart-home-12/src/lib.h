#pragma once

using OutletHandle = void*;

extern "C" {
    OutletHandle allocate_outlet(const char*);

    double get_power(OutletHandle);

    int get_switch(OutletHandle);
    void set_switch(OutletHandle, int);

    const char* get_name(OutletHandle);
    void set_name(OutletHandle, const char*);
}
