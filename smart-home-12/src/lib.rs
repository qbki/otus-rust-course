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
    let ptr = unsafe { transmute::<Box<SmartOutlet>, OutletHandle>(outlet) };
    ptr
}

#[no_mangle]
pub extern "C" fn terminate_outlet(raw_outlet: *mut OutletHandle) {
    let value = unsafe { transmute::<OutletHandle, Box<SmartOutlet>>(*raw_outlet) };
    unsafe { *raw_outlet = std::ptr::null_mut() };
    drop(value);
}


#[no_mangle]
pub extern "C" fn get_power(outlet: OutletHandle) -> libc::c_double {
    let outlet: Box<SmartOutlet> = outlet.into();
    let power = outlet.get_power();
    forget(outlet);
    power
}

#[no_mangle]
pub extern "C" fn get_switch(outlet: OutletHandle) -> libc::c_int {
    let outlet: Box<SmartOutlet> = outlet.into();
    let switch = match outlet.get_switch() {
        SwitchStatusEnum::On => 1,
        SwitchStatusEnum::Off => 0,
    };
    forget(outlet);
    switch
}

#[no_mangle]
pub extern "C" fn set_switch(outlet: OutletHandle, value: libc::c_int) {
    let mut outlet: Box<SmartOutlet> = outlet.into();
    let switch = if value == 0 { SwitchStatusEnum::Off } else { SwitchStatusEnum::On };
    outlet.set_switch(switch);
    forget(outlet);
}

#[no_mangle]
pub extern "C" fn report(outlet: OutletHandle) -> *const libc::c_char {
    let outlet: Box<SmartOutlet> = outlet.into();
    let c_string = ffi::CString::new(outlet.report_to_string()).unwrap();
    forget(outlet);
    c_string.into_raw()
}

impl From<OutletHandle> for Box<SmartOutlet> {
    fn from(handle: OutletHandle) -> Self {
        unsafe { transmute::<OutletHandle, Box<SmartOutlet>>(handle) }
    }
}
