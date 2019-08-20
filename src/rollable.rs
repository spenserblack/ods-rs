use rand::Rng;

/// Defines a type that can be rolled for.
/// Implement this trait on a type you would like to roll for.
///
/// # Example
///
/// ```
/// use one_d_six::{
///     Die,
///     Rollable,
///     quickroll,
/// };
///
/// #[derive(Clone, Copy, Debug)]
/// enum Shapes {
///     Triangle,
///     Square,
///     Circle,
/// }
///
/// impl Rollable for Shapes {
///     // We're ignoring max since we don't need a maximum for this example
///     fn roll(_max: Shapes) -> Shapes {
///         let roll_result: u8 = quickroll("1d3");
///         match roll_result {
///             1 => Shapes::Triangle,
///             2 => Shapes::Square,
///             3 => Shapes::Circle,
///             _ => unreachable!(),
///         }
///     }
/// }
///
/// // We still need a maximum to satisfy Rollable::roll requirements
/// let max = Shapes::Circle;
/// let mut shape_roller = Die::new(max);
/// println!("You rolled {:?}!", shape_roller.roll());
/// ```
pub trait Rollable: Copy {
    fn roll(max: Self) -> Self;
}

impl Rollable for u8 {
    fn roll(max: u8) -> u8 {
        rand::thread_rng().gen_range(0, max) + 1
    }
}
impl Rollable for u16 {
    fn roll(max: u16) -> u16 {
        rand::thread_rng().gen_range(0, max) + 1
    }
}
impl Rollable for u32 {
    fn roll(max: u32) -> u32 {
        rand::thread_rng().gen_range(0, max) + 1
    }
}
impl Rollable for u64 {
    fn roll(max: u64) -> u64 {
        rand::thread_rng().gen_range(0, max) + 1
    }
}
impl Rollable for u128 {
    fn roll(max: u128) -> u128 {
        rand::thread_rng().gen_range(0, max) + 1
    }
}
impl Rollable for usize {
    fn roll(max: usize) -> usize {
        rand::thread_rng().gen_range(0, max) + 1
    }
}
