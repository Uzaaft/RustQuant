// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Simple quote type.
pub struct SimpleQuote {
    value: Option<f64>,
}

impl SimpleQuote {
    pub fn new(value: Option<f64>) -> Self {
        SimpleQuote { value }
    }

    pub fn set_value(&mut self, value: Option<f64>) -> f64 {
        let diff = match (&self.value, &value) {
            (Some(old_value), Some(new_value)) => new_value - old_value,
            _ => 0.0,
        };

        if diff != 0.0 {
            self.value = value;
            // Assuming notify_observers() function exists and does what it's supposed to
            // self.notify_observers();
        }

        diff
    }

    pub fn reset(&mut self) {
        self.set_value(None);
    }
}

impl Quote for SimpleQuote {
    fn value(&self) -> Option<f64> {
        self.value
    }

    fn is_valid(&self) -> bool {
        self.value.is_some()
    }
}
