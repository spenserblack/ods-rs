use std::fmt;
use std::ops::Add;
use std::str::FromStr;

use crate::{
    DiceTotal,
    Die,
    Rollable,
};

/// A Handful of dice.
///
/// # Examples
///
/// ```
/// use one_d_six::Dice;
///
/// let mut dice: Dice = "3d6".parse().unwrap();
///
/// assert!(dice.roll_all().total() >= 3);
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
pub struct Dice<T: Rollable = u32> {
    dice: Vec<Die<T>>,
}

impl<T: Rollable> Add for Dice<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut dice: Vec<Die<T>> = Vec::new();
        for die in self.dice.into_iter() {
            dice.push(die);
        }
        for die in other.dice.into_iter() {
            dice.push(die);
        }
        Dice { dice }
    }
}

impl<T: Rollable> FromStr for Dice<T> where T: FromStr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dice_amount, dice_faces): (usize, T) = {
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

impl<T: Rollable> fmt::Display for Dice<T>
where
    T: DiceTotal<T>,
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.total())
    }
}

impl<T: Rollable> fmt::Debug for Dice<T> where T: fmt::Display {
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

impl<T: Rollable> Dice<T> {
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
    /// let dice: Dice = Dice::new(3, 6);
    /// ```
    pub fn new(dice: usize, faces: T) -> Self {
        let dice = {
            let mut v: Vec<Die<T>> = Vec::with_capacity(dice);
            for _ in 0..dice {
                v.push(Die::new(faces));
            }
            v
        };

        Dice { dice }
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
    /// let dice: Dice = {
    ///     let dice = [
    ///         Die::new(6),
    ///         Die::new(6),
    ///         Die::new(4),
    ///     ];
    ///     Dice::from(Box::new(dice))
    /// };
    /// ```
    pub fn from(dice: Box<[Die<T>]>) -> Self {
        let dice = dice.into_vec();

        Dice { dice }
    }

    /// Gets the current face of each die in the dice set.
    ///
    /// # Example
    ///
    /// ```
    /// use one_d_six::Dice;
    ///
    /// let four_coins: Dice = Dice::new(4, 2);
    ///
    /// for val in four_coins.current_faces().iter() {
    ///     assert!(val == &1 || val == &2);
    /// }
    /// ```
    pub fn current_faces(&self) -> Vec<T> {
        self.dice.iter().map(|die| die.current_face()).collect()
    }

    /// Rolls all dice and returns self.
    ///
    /// # Example
    ///
    /// ```
    /// use one_d_six::Dice;
    ///
    /// let mut ten_d_4 = Dice::new(10, 4);
    ///
    /// for val in ten_d_4.roll_all().current_faces().iter() {
    ///     let val: u32 = *val;
    ///     assert!(val >= 1);
    ///     assert!(val <= 4);
    /// }
    /// ```
    pub fn roll_all(&mut self) -> &Self {
        let iter = self.dice.iter_mut().map(|die| {
            die.roll();
        });
        for _ in iter {}
        self
    }

    /// Gets the total of the current faces of the dice.
    ///
    /// # Example
    ///
    /// ```
    /// use one_d_six::Dice;
    ///
    /// let two_d_4: Dice = Dice::new(2, 4);
    ///
    /// assert!(two_d_4.total() >= 2);
    /// assert!(two_d_4.total() <= 8);
    /// ```
    pub fn total(&self) -> T
    where
        T: DiceTotal<T>,
    {
        T::dice_total(self.current_faces())
    }
}
