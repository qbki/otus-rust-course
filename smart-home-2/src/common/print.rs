pub const PRINT_OFFSET: &'static str  = "    ";

pub trait Print {
    fn print(&self, depth: usize);
}
