# Comparação em relação a performance do __for__ loop em C e em __rust__

## Question

```c
for (int i = 0; i < 10; i++){

/* loop body */
}
```

Rust code

```rust
for i in 0..10{

/* loop body */

}
```

thinking about performance, can I say C code is better ?
what 0..10 really does ?

## Answer

Before optimizations: surely

After optimizations: they're almost surely the same

First, a __for__ loop desugars to something like:

```rust
let iter = IntoIterator::into_iter(0..10);
loop {
    match Iterator::next(iter) {
        Some(i) => { /* loop body */ }
        None => break,
    }
}

```

__0..10__ is an __Iterator__, so __IntoIterator::into_iter__ is a noop and will surely be inlined  
__Iterator::next__ is what's actually doing all the work. For the __Range__ (i.e. the type of __0..10__, which may be different if you use __..=__ or you omit one of the two bounds) type it is implemented as:

```rust
#[inline]
fn next(&mut self) -> Option<A> {
    if self.start < self.end {
        // SAFETY: just checked precondition
        let n = unsafe { Step::forward_unchecked(self.start.clone(), 1) };
        Some(mem::replace(&mut self.start, n))
    } else {
        None
    }
}
```

The __start__ and __end__ fields start as __0__ and __10__ and updated when the iterator advances. While used as normal __Iterator__ (not __DoubleEndedIterator__) __end__ will never be modified, so it will probably be inlined. The function itself will also be probably inlined. And since the function is generating an __Option__ and then you directly match on it, the __match__ can be skipped and its arms placed when the __Some__/__None__ are created. So the loop could probably get optimized to something like:

```rust
let iter = IntoIterator::into_iter(0..10);
loop {
    if iter.start < iter.end {
        let n = unsafe { Step::forward_unchecked(iter.start.clone(), 1) };
        let i = mem::replace(&mut iter.start, n);
        /* loop body */
    } else {
        break
    }
}
```

Finally, the __Step::forward_unchecked(iter.start.clone(), 1)__ is there to make the iteration over a __Range__ generic. In our case the __Range__ is over some numeric type, let's say __i32__, so the __.clone()__ is just a simple copy, and __Step::forward_unchecked__ is implemented as:

```rust
 #[inline]
    unsafe fn forward_unchecked(start: Self, n: usize) -> Self {
        // SAFETY: the caller has to guarantee that `start + n` doesn't overflow.
        unsafe { start.unchecked_add(n as Self) }
    }
```

Where __unchecked_add__ is actually implemented as a compiler intrinsics that compile to an addition that's guaranteed to not overflow (so that the optimizer can make additional optimizations knowing that!).

The __mem::replace(&mut iter.start, n)__ actually just reads the value in __iter.start__, replaces it with __n__ and returns the value read. The optimizer should actually be able to swap some read and write.

Finally, after all the optimizations, inling, reordering, ecc ecc you get something like:

```rust
let i = 0;
loop {
    if i < 10 {
        /* loop body */
        unsafe { i = i.unchecked_add(1) };
    } else {
        break
    }
}
```

Which is pretty much the same as the C version.

by: _Skifire13_. (a cool reddit user)
