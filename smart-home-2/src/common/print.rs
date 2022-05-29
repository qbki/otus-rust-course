pub const PRINT_OFFSET: &str = "    ";

pub trait Print {
    fn print(&self, depth: usize);
}
