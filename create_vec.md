# Passagem por valor e por referência

## Question

```rust
fn create_vec() -> Vec<i32> {
    let mut v = Vec::new();
    v.push(1);

    v
}

```

it is cheap ? Like when we create function with returns a std::vector\<int>* in C++ If not cheap, how create a function with returns a Vec in cheap way ?

## Answer

Rust never performs deep copies without an explicit call to __.clone()__. If you just move it, it's cheap.
