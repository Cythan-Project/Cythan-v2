# Cythan v3
 Cythan is an abstract machine that has been created to be simpler than Turing's one.
 This is the Rust implementation of Cythan.

## Why Rust ?
 - Blazingly fast performances 
 - Low memory foot-print
 - Great ecosystem
 - Concurrency
 - Memory safety
 - WASM compilable

## How to use Cythan in a project

#### Cargo.toml
```
[dependencies]
cythan = "*"
```

#### Example
```rust
let mut cythan = Cythan::new(vec![12,78,15,21,20]);
for _ in 0..20 {
    cythan.next();
}
println!("{}",cythan);
```
## Have found a bug, want to contribute or had an idea ?
Go in the issue section and leave an issue or fix one!
