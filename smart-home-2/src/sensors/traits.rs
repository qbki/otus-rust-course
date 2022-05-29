pub trait Sensor<T> {
    fn sample(&self) -> T;
}
