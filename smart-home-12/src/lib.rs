mod common;
mod smart_outlet;

use smart_outlet::*;
use common::{SwitchStatusEnum, Report};
use std::ffi;
use std::mem::{forget, transmute};

type OutletHandle = *mut libc::c_void;

#[no_mangle]
pub extern "C" fn allocate_outlet() -> OutletHandle {
    let outlet = Box::new(SmartOutlet::new());
    unsafe { transmute::<Box<SmartOutlet>, OutletHandle>(outlet) }
}

#[no_mangle]
/// # Safety
///
/// This function should not be called before the allocate_outlet.
pub unsafe extern "C" fn terminate_outlet(raw_outlet: *mut OutletHandle) {
    let value = transmute::<OutletHandle, Box<SmartOutlet>>(*raw_outlet);
    *raw_outlet = std::ptr::null_mut();
    drop(value);
}


#[no_mangle]
pub extern "C" fn get_power(handle: OutletHandle) -> libc::c_double {
    let outlet = to_outlet(handle);
    let power = outlet.get_power();
    forget(outlet);
    power
}

#[no_mangle]
pub extern "C" fn get_switch(handle: OutletHandle) -> libc::c_int {
    let outlet = to_outlet(handle);
    let switch = match outlet.get_switch() {
        SwitchStatusEnum::On => 1,
        SwitchStatusEnum::Off => 0,
    };
    forget(outlet);
    switch
}

#[no_mangle]
pub extern "C" fn set_switch(handle: OutletHandle, value: libc::c_int) {
    let mut outlet = to_outlet(handle);
    let switch = if value == 0 { SwitchStatusEnum::Off } else { SwitchStatusEnum::On };
    outlet.set_switch(switch);
    forget(outlet);
}

#[no_mangle]
pub extern "C" fn report(handle: OutletHandle) -> *const libc::c_char {
    let outlet = to_outlet(handle);
    let c_string = ffi::CString::new(outlet.report_to_string()).unwrap();
    forget(outlet);
    c_string.into_raw()
}

fn to_outlet(handle: OutletHandle) -> Box<SmartOutlet> {
    unsafe { transmute::<OutletHandle, Box<SmartOutlet>>(handle) }
}
