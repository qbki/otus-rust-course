#[macro_export]
macro_rules! accessors {
    ($getter_name:ident, $setter_name: ident, $field: ident, String) => {
        pub fn $getter_name(&self) -> &str {
            &self.$field.as_ref()
        }

        pub fn $setter_name(&mut self, value: &str) -> &Self {
            self.$field = value.to_string();
            self
        }
    };
    ($getter_name:ident, $setter_name: ident, $field: ident, $type:ty) => {
        pub fn $getter_name(&self) -> $type {
            let arc = Arc::clone(&self.$field);
            let lock = arc.lock().unwrap();
            *lock
        }

        pub fn $setter_name(&self, value: $type) -> &Self {
            *self.$field.lock().unwrap() = value;
            self
        }
    };
}
