#[macro_export]
macro_rules! accessors {
    ($getter_name:ident, $setter_name: ident, $field: ident, $type:ty) => {
        pub fn $getter_name(&self) -> $type {
            self.$field.get()
        }

        pub fn $setter_name(&self, value: $type) {
            self.$field.set(value);
        }
    };
}
