mod lib {
    use std::ffi::CString;

    type OutletHandlePtr = *mut libc::c_void; 

    pub struct OutletHandle(OutletHandlePtr);

    #[link(name = "smart_lib_12")]
    extern {
        pub fn allocate_outlet(name: *const libc::c_char) -> OutletHandlePtr;
        pub fn terminate_outlet(outlet: OutletHandlePtr);

        pub fn get_name(outlet: OutletHandlePtr) -> *mut libc::c_char;
        pub fn set_name(outlet: OutletHandlePtr, name: *const libc::c_char);

        pub fn get_power(outlet: OutletHandlePtr) -> libc::c_double;

        pub fn get_switch(outlet: OutletHandlePtr) -> libc::c_int;
        pub fn set_switch(outlet: OutletHandlePtr, value: libc::c_int);

        pub fn report(outlet: OutletHandlePtr) -> *mut libc::c_char;
    }

    //impl Drop for OutletHandle {
        //fn drop(&mut self) {
            //if self.0 != std::ptr::null_mut() {
                //unsafe { terminate_outlet(self.0) };
            //}
        //}
    //}

    impl OutletHandle {
        pub fn new(name: &str) -> OutletHandle {
            let string = CString::new(name).unwrap();
            let outlet = unsafe { allocate_outlet(string.as_ptr()) };
            OutletHandle(outlet)
        }

        pub fn get_name(&self) -> String {
            unsafe { CString::from_raw(get_name(self.0)).into_string().unwrap() }
        }

        pub fn set_name(&self, name: &str) {
            let string = CString::new(name).unwrap();
            unsafe { set_name(self.0, string.as_ptr()) };
        }

        pub fn get_power(&self) -> f64 {
            unsafe { get_power(self.0) }
        }

        pub fn get_switch(&self) -> bool {
            let value = unsafe { get_switch(self.0) };
            value != 0
        }

        pub fn set_switch(&self, value: bool) {
            let value = value as i32;
            unsafe { set_switch(self.0, value) };
        }

        pub fn report(&self) -> String {
            let ptr = unsafe { report(self.0) };
            unsafe { CString::from_raw(ptr).into_string().unwrap() }
        }
    }
}

fn main() {
    let outlet = lib::OutletHandle::new("Fridge");

    println!("*** Properties ***");
    println!("Name: {}", outlet.get_name());
    println!("Power: {}", outlet.get_power());
    println!("Switch: {}", outlet.get_switch());

    println!("*** Report ***");
    println!("{}", outlet.report());

    // outlet.set_name("Light");
    outlet.set_switch(true);

    println!("*** Modified Report ***");
    println!("{}", outlet.report());
}
