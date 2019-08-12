//! Provides utilities for rolling dice.
//!
//! # Examples
//!
//! ```
//! use one_d_six::quickroll;
//!
//! if quickroll("1d2") == 1 {
//!     println!("Heads!");
//! } else {
//!     println!("Tails!");
//! }
//! ```
//!
//! ```
//! use one_d_six::Dice;
//!
//! // 3d6
//! let set_1 = Dice::new(3, 6);
//! // 2d4
//! let set_2: Dice = "2d4".parse().unwrap();
//!
//! // 3d6 + 2d4
//! let dice = set_1 + set_2;
//!
//! // Each set of dice starts pre-rolled
//! let roll = dice.total();
//!
//! println!("Result of 3d6 + 2d4 roll: {}", roll);
//! ```
use std::fmt;
use std::ops::Add;
use rand::Rng;

/// Attempts to roll dice based on a *1d6* style string.
///
/// # Example
///
/// ```
/// use one_d_six::try_quickroll;
///
/// if let Ok(roll) = try_quickroll("1d6") {
///     assert!(roll >= 1);
///     assert!(roll <= 6);
/// } else {
///     unreachable!();
/// }
/// ```
pub fn try_quickroll(dice_format: &str) -> Result<u32, String> {
    let dice: Dice = dice_format.parse()?;
    Ok(dice.total())
}

/// Rolls dice based on a *1d6* style string.
///
/// # Example
///
/// ```
/// use one_d_six::quickroll;
///
/// let coin_flip = quickroll("1d2");
///
/// assert!(coin_flip == 1 || coin_flip == 2);
/// ```
///
/// # Panics
///
/// Panics if `dice_format` is in an improper format.
pub fn quickroll(dice_format: &str) -> u32 {
    let dice: Dice = dice_format.parse().unwrap();
    dice.total()
}

/// Represents a single die.
/// Has an initial random value.
///
/// # Examples
/// ## Adding roll values
/// ```
/// use one_d_six::Die;
///
/// let mut d4 = Die::new(4);
/// let mut d6 = Die::new(6);
///
/// let d4_result: u32 = d4.roll();
/// let d6_result: u32 = d6.roll();
/// let result = d4_result + d6_result;
///
/// assert!(result >= 2);
/// assert!(result <= 10);
/// ```
///
/// ## Adding current values
/// ```
/// use one_d_six::Die;
///
/// let d4 = Die::new(4);
/// let d6 = Die::new(6);
///
/// let d4_result: u32 = d4.current_face();
/// let d6_result: u32 = d6.current_face();
/// let result = d4_result + d6_result;
///
/// assert!(result >= 2);
/// assert!(result <= 10);
/// ```
///
/// ## Adding dice directly
/// ```
/// use one_d_six::Die;
///
/// let d4 = Die::new(4);
/// let d6 = Die::new(6);
///
/// let result = d4 + d6;
///
/// assert!(result >= 2);
/// assert!(result <= 10);
/// ```
pub struct Die {
    faces: u32,
    current_value: u32,
}

/// A Handful of dice.
///
/// # Examples
///
/// ```
/// use one_d_six::Dice;
///
/// let mut dice: Dice = "3d6".parse().unwrap();
///
/// dice = dice.roll_all();
///
/// assert!(dice.total() >= 3);
/// assert!(dice.total() <= 18);
/// ```
///
/// ## Adding two collections of dice
///
/// ```
/// use one_d_six::Dice;
///
/// let one_d6: Dice = "1d6".parse().unwrap();
/// let three_d4: Dice = Dice::new(3, 4);
///
/// let dice = one_d6 + three_d4;
///
/// assert!(dice.total() >= 4);
/// assert!(dice.total() <= 18);
/// ```
pub struct Dice {
    dice: Vec<Die>,
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

impl Add for Die {
    type Output = u32;

    fn add(self, other: Self) -> Self::Output {
        self.current_value + other.current_value
    }
}

impl Dice {
    /// Creates a new set of dice.
    /// Each die in the set has an initial starting value.
    /// Only allows dice of same type. No mixture of d4 and d6.
    ///
    /// # Example
    ///
    /// ```
    /// use one_d_six::Dice;
    ///
    /// // Creates 3d6 dice collection
    /// let dice = Dice::new(3, 6);
    /// ```
    pub fn new(dice: usize, faces: u32) -> Self {
        let dice = {
            let mut v: Vec<Die> = Vec::with_capacity(dice);
            for _ in 0..dice {
                v.push(Die::new(faces));
            }
            v
        };

        Dice {
            dice
        }
    }

    /// Creates a set of dice from a `Vec<Die>`.
    /// Allows for mixture of Die types (d4, d6, etc.).
    ///
    /// # Example
    ///
    /// ```
    /// use one_d_six::{
    ///     Dice,
    ///     Die,
    /// };
    ///
    /// // Creates 2d6 + 1d4 dice collection
    /// let dice = {
    ///     let dice = [
    ///         Die::new(6),
    ///         Die::new(6),
    ///         Die::new(4),
    ///     ];
    ///     Dice::from(Box::new(dice))
    /// };
    /// ```
    pub fn from(dice: Box<[Die]>) -> Self {
        let dice = dice.into_vec();

        Dice {
            dice
        }
    }

    /// Gets the current face of each die in the dice set.
    ///
    /// # Example
    ///
    /// ```
    /// use one_d_six::Dice;
    ///
    /// let four_coins = Dice::new(4, 2);
    ///
    /// for val in four_coins.current_faces().iter() {
    ///     assert!(val == &1 || val == &2);
    /// }
    /// ```
    pub fn current_faces(&self) -> Vec<u32> {
        self.dice.iter().map(|die| {
            die.current_face()
        }).collect()
    }

    /// Rolls all dice and gets the face of each one.
    ///
    /// # Example
    ///
    /// ```
    /// use one_d_six::Dice;
    ///
    /// let mut ten_d_4 = Dice::new(10, 4);
    ///
    /// for val in ten_d_4.roll_all().current_faces().iter() {
    ///     let val = *val;
    ///     assert!(val >= 1);
    ///     assert!(val <= 4);
    /// }
    /// ```
    pub fn roll_all(mut self) -> Self {
        self.dice.iter_mut().map(|die| {
            die.roll();
        });
        self
    }

    /// Gets the total of the current faces of the dice.
    ///
    /// # Example
    ///
    /// ```
    /// use one_d_six::Dice;
    ///
    /// let two_d_4 = Dice::new(2, 4);
    ///
    /// assert!(two_d_4.total() >= 2);
    /// assert!(two_d_4.total() <= 8);
    /// ```
    pub fn total(&self) -> u32 {
        self.current_faces().iter().sum()
    }
}

impl Add for Dice {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut dice: Vec<Die> = Vec::new();
        for die in self.dice.into_iter() {
            dice.push(die);
        }
        for die in other.dice.into_iter() {
            dice.push(die);
        }
        Dice {
            dice,
        }
    }
}

impl std::str::FromStr for Dice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dice_amount, dice_faces): (usize, u32) = {
            let mut s = s.split('d');
            let values = if let (Some(d), Some(f)) = (s.next(), s.next()) {
                (d.parse(), f.parse())
            } else {
                return Err(String::from("Missing 'd'"));
            };

            if let (Ok(d), Ok(f)) = values {
                (d, f)
            } else {
                return Err(String::from("Improper dice format"));
            }
        };
        Ok(Dice::new(dice_amount, dice_faces))
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.total())
    }
}

impl fmt::Debug for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iter = self.dice.iter();
        let first = match iter.next() {
            Some(d) => d,
            None => return Err(fmt::Error),
        };
        if let Err(e) = write!(f, "{}", first.current_face()) {
            return Err(e);
        }

        for die in iter {
            if let Err(e) = write!(f, " {}", die.current_face()) {
                return Err(e);
            }
        }
        Ok(())
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

    #[test]
    fn add_die() {
        for _ in 0..100 {
            let penny = Die::new(2);
            let quarter = Die::new(2);

            let sum = penny + quarter;

            assert!(sum >= 2);
            assert!(sum <= 4);
        }
    }

    #[test]
    fn current_faces() {
        for _ in 0..100 {
            let dice = Dice::new(3, 6);

            let sum: u32 = dice.current_faces().iter().sum();

            assert!(sum >= 3);
            assert!(sum <= 18);
        }
    }

    #[test]
    fn roll_all() {
        for _ in 0..100 {
            let mut dice = Dice::new(4, 2);

            let sum: u32 = dice.roll_all().current_faces().iter().sum();

            assert!(sum >= 4);
            assert!(sum <= 8);
        }
    }

    #[test]
    fn total() {
        for _ in 0..100 {
            let dice = Dice::new(2, 3);
            let total = dice.total();

            assert!(total >= 2);
            assert!(total <= 6);
        }
    }

    #[test]
    fn add_dice() {
        let one_d_6 = Dice::new(1, 6);
        let two_d_4 = Dice::new(2, 4);
        let mut dice = one_d_6 + two_d_4;

        for _ in 0..100 {
            dice = dice.roll_all();
            let total = dice.total();
            assert!(total >= 2);
            assert!(total <= 14);
        }
    }

    #[test]
    fn dice_from_str() {
        let mut dice: Dice = "3d4".parse().unwrap();

        for _ in 0..100 {
            dice = dice.roll_all();
            let total = dice.total();
            assert!(total >= 3);
            assert!(total <= 12);
        }
    }
}
