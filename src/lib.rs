//! `Cd`: A "smart pointer" that tracks changes to the data it owns.
//! 
//! ## Usage
//! ```
//! use changed::Cd;
//! 
//! // Create the change tracker with an i32
//! let mut test: Cd<i32> = Cd::new(20);
//! 
//! // Mutate it (calling deref_mut through the *)
//! *test += 5;
//! 
//! // changed() reports whether or not it was changed
//! assert!(Cd::changed(&test));
//! 
//! // Reset the tracker back to false
//! Cd::reset(&mut test);
//! 
//! // Read the data
//! assert_eq!(*test, 25);
//! 
//! // That didn't trip the change detection!
//! assert!(!Cd::changed(&test));
//! ```
//! 
//! ## How it works
//! Technically, it doesn't track changes. It tracks calls to `deref_mut()`
//! so it is entirely possible to call `deref_mut()` and not change it, giving a false positive.
//! 
//! Along with that, there is a function to mutate a `Cd` without tripping change detection. 

use std::ops::{Deref, DerefMut};
use std::fmt;

/// Cd: Change Detection
///
/// Start by creating one with [`new()`](Cd::new()).
pub struct Cd<T> {
    data: T,
    changed: bool,
}

impl<T> Cd<T> {
    /// Create a new Cd with data.
    /// It is initialized to false for change detection.
    ///
    /// ```
    /// use changed::Cd;
    /// let cd = Cd::new(5);
    /// ```
    pub fn new(data: T) -> Cd<T> {
        Cd {
            data,
            changed: false,
        }
    }

    /// Create a new Cd with data.
    /// It is initialized to true for change detection.
    /// ```
    /// use changed::Cd;
    /// let cd = Cd::new_true(5);
    /// assert!(Cd::changed(&cd));
    /// ```
    pub fn new_true(data: T) -> Cd<T> {
        Cd {
            data,
            changed: true,
        }
    }


    /// Reset the change tracking to false.
    /// ```
    /// use changed::Cd;
    /// let mut cd = Cd::new_true(5);
    /// Cd::reset(&mut cd);
    /// assert!(!Cd::changed(&cd));
    /// ```
    pub fn reset(this: &mut Cd<T>) {
        this.changed = false;
    }

    /// Take the data out of the Cd.
    /// Consumes this and returns data.
    /// ```
    /// use changed::Cd;
    /// let cd = Cd::new(5);
    /// let data = Cd::take(cd);
    /// // Error: cd has been moved.
    /// // Cd::changed(&cd);
    /// ```
    pub fn take(this: Cd<T>) -> T {
        this.data
    }

    /// Check if the Cd has been changed since the last call to reset (or created.)
    /// ```
    /// use changed::Cd;
    /// let mut cd = Cd::new(5);
    /// assert!(!Cd::changed(&cd));
    /// *cd += 5;
    /// assert!(Cd::changed(&cd));
    /// ```
    pub fn changed(this: &Cd<T>) -> bool {
        this.changed
    }

    /// Mutate the Cd without tripping change detection.
    ///
    /// ```
    /// use changed::Cd;
    /// let mut cd = Cd::new(5);
    /// *Cd::silent_mut(&mut cd) += 5;
    /// assert!(!Cd::changed(&cd));
    /// ```
    pub fn silent_mut(this: &mut Cd<T>) -> &mut T {
        &mut this.data
    }

}

/// deref does not trip change detection.
/// ```
/// use changed::Cd;
/// let cd = Cd::new(5);
/// assert_eq!(*cd, 5); // deref for == 5
/// assert!(!Cd::changed(&cd)); // .changed() is false
/// ```
impl<T> Deref for Cd<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// deref_mut trips change detection.
/// ```
/// use changed::Cd;
/// let mut cd = Cd::new(5);
/// *cd += 5; // deref_mut for add assign
/// assert_eq!(*cd, 10);
/// assert!(Cd::changed(&cd)); // .changed() is true
/// ```
impl<T> DerefMut for Cd<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.changed = true;
        &mut self.data
    }
}

/// Impl default where the data impls default. Change detection is initialized to false.
/// ```
/// use changed::Cd;
/// // 0 is default for i32.
/// let zero: Cd<i32> = Cd::default();
/// assert!(!Cd::changed(&zero));
/// ```
impl<T: Default> Default for Cd<T> {
    fn default() -> Self {
        Cd::new(T::default())
    }
}

impl <T: Clone> Clone for Cd<T> {
    fn clone(&self) -> Self {
        Cd {
            data: self.data.clone(),
            changed: self.changed
        }
    }
}

impl <T: fmt::Debug> fmt::Debug for Cd<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cd").field("data", &self.data)
                            .field("changed", &self.changed).finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::Cd;

    #[test]
    fn it_works() {
        let mut cd = Cd::new(15);
        *cd += 5;
        assert!(Cd::changed(&cd));
        Cd::reset(&mut cd);
        assert_eq!(*cd, 20);
        assert!(!Cd::changed(&cd));
    }
}
