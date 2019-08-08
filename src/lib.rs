use rand::Rng;

/// Represents a single die.
pub struct Die {
    faces: u32,
    current_value: u32,
}

impl Die {
    /// Creates a single die with the specified number of faces.
    ///
    /// # Example
    ///
    /// ```
    /// use one_d_six::Die;
    ///
    /// let coin = Die::new(2);
    /// ```
    pub fn new(faces: u32) -> Self {
        let mut die = Die {
            faces,
            current_value: 1,
        };
        die.roll();
        die
    }
    /// Rolls a single die.
    ///
    /// # Example
    ///
    /// ```
    /// use one_d_six::Die;
    ///
    /// let mut d6 = Die::new(6);
    ///
    /// assert!(d6.roll() >= 1);
    /// assert!(d6.current_face() <= 6);
    /// ```
    pub fn roll(&mut self) -> u32 {
        let r: u32 = rand::random();
        self.current_value = (r % self.faces) + 1;
        self.current_value
    }
    /// Gets the current value of the die.
    ///
    /// # Example
    ///
    /// ```
    /// use one_d_six::Die;
    ///
    /// let d4 = Die::new(4);
    ///
    /// assert!(d4.current_face() >= 1);
    /// assert!(d4.current_face() <= 4);
    /// ```
    pub fn current_face(&self) -> u32 {
        self.current_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_face() {
        let coin = Die::new(2);

        for _ in 0..100 {
            assert!(coin.current_face() >= 1);
            assert!(coin.current_face() <= 2);
        }
    }

    #[test]
    fn roll() {
        let mut d12 = Die::new(12);

        for _ in 0..100 {
            d12.roll();
            assert!(d12.current_face() >= 1);
            assert!(d12.current_face() <= 12);
        }
    }
}
