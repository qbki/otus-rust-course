mod common;
mod smart_outlet;

use smart_outlet::*;
use common::SwitchStatusEnum;
use std::ffi;

type OutletHandle = *mut libc::c_void;

#[no_mangle]
pub extern "C" fn allocate_outlet(name: *const libc::c_char) -> OutletHandle {
    let c_str_name = unsafe { ffi::CStr::from_ptr(name) };
    let outlet = Box::new(SmartOutlet::new(c_str_name.to_str().unwrap_or("")));
    let ptr = Box::into_raw(outlet);
    ptr as *mut _ as OutletHandle
}

#[no_mangle]
pub extern "C" fn get_name(outlet: OutletHandle) -> *const libc::c_char {
    let outlet: Box<SmartOutlet> = outlet.into();
    let c_string = ffi::CString::new(outlet.get_name()).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn set_name(outlet: OutletHandle, name: *mut libc::c_char) {
    let name = unsafe { 
        let c_string = ffi::CString::from_raw(name);
        c_string.into_string().unwrap()
    };
    let mut outlet: Box<SmartOutlet> = outlet.into();
    outlet.set_name(&name);
}

#[no_mangle]
pub extern "C" fn get_power(outlet: OutletHandle) -> libc::c_double {
    let outlet: Box<SmartOutlet> = outlet.into();
    outlet.get_power()
}

#[no_mangle]
pub extern "C" fn get_switch(outlet: OutletHandle) -> libc::c_int {
    let outlet: Box<SmartOutlet> = outlet.into();
    match outlet.get_switch() {
        SwitchStatusEnum::On => 1,
        SwitchStatusEnum::Off => 0,
    }
}

#[no_mangle]
pub extern "C" fn set_switch(outlet: OutletHandle, value: libc::c_int) {
    let mut outlet: Box<SmartOutlet> = outlet.into();
    let switch = if value == 0 { SwitchStatusEnum::Off } else { SwitchStatusEnum::On };
    outlet.set_switch(switch);
}

impl From<OutletHandle> for Box<SmartOutlet> {
    fn from(handle: OutletHandle) -> Self {
        unsafe { Box::from_raw(handle as *mut _) }
    }
}
