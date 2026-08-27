# LeetCode Through Rust

LeetCode solutions written in Rust.

## Structure

Each LeetCode problem is an independent Cargo binary under `src/bin/`.

```text
src/bin/
└── 0001-two-sum.rs
```

## Workflow

1. Solve the problem on LeetCode.
2. Add the solution as XXXX-problem-name.rs.
3. Add struct Solution; or other minimal changes needed to compile locally.
```rust
struct Solution;

impl Solution {
    // solution
}
```
4. (Optional) Add tests only when useful for debugging.
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_case() {
        // ...
    }
}
```
```bash
cargo test --bin 0001-two-sum
```
5. (Optional) Run the problem independently with Cargo with main if required.
```rust
fn main() {
    println!("Hello, world!");
}
```
```bash
cargo run --bin 0001-two-sum
```
6. Commit the solution.


