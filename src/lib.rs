//! `Cd`: A "smart pointer" that tracks changes to the data it owns.
//! 
//! ## Usage
//! Put changed in your Cargo.toml like so
//! ```txt
//! changed = 0.1.0
//! ```
//! Then in your code:
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
//! assert!(test.changed());
//! 
//! // Reset the tracker back to false
//! test.reset();
//! 
//! // Read the data
//! assert_eq!(*test, 25);
//! 
//! // That didn't trip the change detection!
//! assert!(!test.changed());
//! ```
//! 
//! ## How it works
//! Technically, it doesn't track changes. It tracks calls to `deref_mut()`
//! so it is entirely possible to call `deref_mut()` and not change it, giving a false positive.
//! 
//! Along with that, there is a function to mutate a `Cd` without tripping change detection. 
//! However, it is marked unsafe, since unsafe declares that there are contracts the compiler can't check.
//! If Cd promises that you can't mutate it without it knowing, then it must uphold that.
//! But in case you really really need to, there is the unsafe `mutate_silently()`.

use std::ops::{Deref, DerefMut};

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
    /// assert!(cd.changed());
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
    /// cd.reset();
    /// assert!(!cd.changed());
    /// ```
    pub fn reset(&mut self) {
        self.changed = false;
    }

    /// Take the data out of the Cd.
    /// Consumes self and returns data.
    /// ```
    /// use changed::Cd;
    /// let cd = Cd::new(5);
    /// let data = cd.take();
    /// // Error: cd has been moved.
    /// // cd.changed();
    /// ```
    pub fn take(self) -> T {
        self.data
    }

    /// Check if the Cd has been changed since the last call to reset (or created.)
    /// ```
    /// use changed::Cd;
    /// let mut cd = Cd::new(5);
    /// assert!(!cd.changed());
    /// *cd += 5;
    /// assert!(cd.changed());
    /// ```
    pub fn changed(&self) -> bool {
        self.changed
    }

    /// Mutate the Cd without tripping change detection.
    ///
    /// # Safety
    /// This is marked unsafe since this library guarantees that Cd cannot be changed
    /// without tripping change detection.
    ///
    /// However, in case you really really need to, 
    /// this function allows you to mutate it without change detection.
    ///
    /// Nothing about this is memory unsafe or type unsafe, it just violates the contracts made by Cd.
    /// ```
    /// use changed::Cd;
    /// let mut cd = Cd::new(5);
    /// unsafe {
    ///     *cd.mutate_silently() += 5;
    /// }
    /// assert!(!cd.changed());
    /// ```
    pub unsafe fn mutate_silently(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T> Deref for Cd<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Cd<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.changed = true;
        &mut self.data
    }
}

impl<T: Default> Default for Cd<T> {
    fn default() -> Self {
        Cd::new(T::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::Cd;

    #[test]
    fn it_works() {
        let mut changed = Cd::new(15);
        *changed += 5;
        assert!(changed.changed);
        changed.reset();
        assert_eq!(*changed, 20);
        assert!(!changed.changed);
    }
}
