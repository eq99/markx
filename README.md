# markx
Markx is a markdown parser built with Rust

# Usage

```rust
use markx::html::mark2html;


fn main(){
    let input = fs::read_to_string("tests/test4.md").unwrap();
    let html = mark2html(&input);
    println!("{:?}", html)
}
```