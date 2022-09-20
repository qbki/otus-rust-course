mod lib {
    use std::ffi::CString;

    type OutletHandlePtr = *mut libc::c_void;

    pub struct OutletHandle(OutletHandlePtr);

    #[link(name = "smart_lib_12")]
    extern "C" {
        pub fn allocate_outlet(name: *const libc::c_char) -> OutletHandlePtr;
        pub fn terminate_outlet(outlet: *mut OutletHandlePtr);

        pub fn get_power(outlet: OutletHandlePtr) -> libc::c_double;

        pub fn get_switch(outlet: OutletHandlePtr) -> libc::c_int;
        pub fn set_switch(outlet: OutletHandlePtr, value: libc::c_int);

        pub fn report(outlet: OutletHandlePtr) -> *mut libc::c_char;
    }

    impl OutletHandle {
        pub fn new(name: &str) -> OutletHandle {
            let string = CString::new(name).unwrap();
            let outlet = unsafe { allocate_outlet(string.as_ptr()) };
            OutletHandle(outlet)
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

    impl Drop for OutletHandle {
        fn drop(&mut self) {
            unsafe { terminate_outlet(&mut self.0) };
        }
    }
}

fn main() {
    let outlet = lib::OutletHandle::new("Fridge");

    println!("*** Properties ***");
    println!("Power: {}", outlet.get_power());
    println!("Switch: {}", outlet.get_switch());

    println!("*** Report ***");
    println!("{}", outlet.report());

    outlet.set_switch(true);

    println!("*** Modified Report ***");
    println!("{}", outlet.report());
}
