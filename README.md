# ct-for

![license](https://img.shields.io/crates/l/ct-for?style=for-the-badge)
[![crates.io](https://img.shields.io/crates/v/ct-for?style=for-the-badge)](https://crates.io/crates/ct-for)
[![ci](https://img.shields.io/github/workflow/status/rob2309/ct-for-rs/Continuous%20Integration?label=CI&style=for-the-badge)](https://github.com/Rob2309/ct-for-rs/actions/workflows/ci.yaml)
[![docs.rs](https://img.shields.io/docsrs/ct-for?style=for-the-badge)](https://docs.rs/ct-for)

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
