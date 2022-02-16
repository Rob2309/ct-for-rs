# ct-for

This crate exposes the `ct-for!()` macro, which can be used to repeat code `n` times with a substitution.

For example:
```rust
let c = 17;
ct_for!(x in ["5", 6, c, vec![5, 6, 7]]
    println!("{:?}", x);
);
```
expands to:
```rust
let c = 17;
println!("{:?}", "5");
println!("{:?}", 6);
println!("{:?}", c);
println!("{:?}", vec![5, 6, 7]);
```

The `ct_for!()` macro can also be nested.

There really isn't much more to it.
