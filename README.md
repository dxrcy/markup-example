# Markup Compiler Example

An example of a custom markup to html compiler.

[Syntax Highlighting VSCode Extension](https://github.com/darccyy/markdown-example-syntax)

```ps1
cargo run -- index.mu
```

```rs
fn main() {
    // Read file
    let file_in = fs::read_to_string("./index.mu").unwrap();

    // Compile
    let file_out = compile(&file_in).unwrap();

    // Write file
    fs::write(out, "./index.html").unwrap();
}
```
