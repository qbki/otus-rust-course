use std::cell::Cell;

use super::traits::Sensor;

pub struct MockedSensor<T> {
    data: Vec<T>,
    index: Cell<usize>,
}

impl<T> MockedSensor<T> {
    pub fn new(data: Vec<T>) -> Self {
        assert!(!data.is_empty());
        Self {
            data,
            index: Cell::new(0),
        }
    }

    fn increment(&self) {
        let index = self.index.get() + 1;
        if index >= self.data.len() {
            self.index.set(0);
        } else {
            self.index.set(index);
        }
    }
}

impl<T> Sensor<T> for MockedSensor<T>
    where T : Copy
{
    fn sample(&self) -> T {
        let index = self.index.get();
        self.increment();
        self.data[index]
    }
}
