`Cd`: A "smart pointer" that tracks changes to the data it owns.

## Usage
Put changed in your Cargo.toml like so
```
changed = 0.1.0
```
Then in your code:
```
use changed::Cd;

// Create the change tracker with an i32
let mut test: Changed<i32> = Changed::new(20);

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

## How it works
Technically, it doesn't track changes. It tracks calls to `deref_mut()`
so it is entirely possible to call `deref_mut()` and not change it, giving a false positive.

Along with that, there is a function to mutate a `Cd` without tripping change detection. 
However, it is marked unsafe, since unsafe declares that there are contracts the compiler can't check.
If Cd promises that you can't mutate it without it knowing, then it must uphold that.
But in case you really really need to, there is the unsafe `mutate_silently()`.
