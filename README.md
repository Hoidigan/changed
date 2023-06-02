`Cd`: A "smart pointer" that tracks changes to the data it owns.

## Usage
```
use changed::Cd;

// Create the change tracker with an i32
let mut test: Cd<i32> = Cd::new(20);

// Mutate it (calling deref_mut through the *)
*test += 5;

// changed() reports whether or not it was changed
assert!(test.changed());

// Reset it the tracker back to false
test.reset();

// Read the data
assert_eq!(*test, 25);

// That didn't trip the change detection!
assert!(!test.changed());
```

## How It Works
Technically, it doesn't track changes. It tracks calls to `deref_mut()` so it is
entirely possible to call `deref_mut()` and not change it, giving a false
positive.

Along with that, there is a function to mutate a `Cd` without tripping change
detection.
