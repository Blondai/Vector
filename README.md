# Vector

This project hardens the build in `Vec` struct with the addition of a `start` and `end`.
This allows the new `Vector` to automatically handle index offsets.

To improve the handling of these structs they are generic.
To implement this functionality onto your own data types one must implement the `Vectorable` trait.
The only real requirement is the `Copy` trait.

The standard wrapper is `OwnedVector`.
This struct owns the underlying `Vec`.
In contrast, the `BorrowedVector` only borrows the `Vec`.
The latter allows the use of constant `BorrowedVector`s.

The planned use case is inside of something like a biometric kernel, where the mortality rates start at an age of 20.
To not have to think about the index shifting one can use these new `Vector`s.

# TODOs

- [ ] Add `del_first` and `del_last` methods.
- [ ] Add `push_first` and `push_last` methods.
- [ ] Add Examples.
- [ ] Add Unit Tests.
- [ ] Add Integration Tests.
