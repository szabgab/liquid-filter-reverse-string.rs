# Liquid filter for Rust to reverse a string

The [liquid](https://crates.io/crates/liquid) crate, the Rust implementation of the [liquid](https://shopify.github.io/liquid/) template
system has many filters to manipulate the data in the template, but AFAIK there is no filter to reverse a string.

This crate was originally developed as a demo on how to add a filter to liquid.

## Usage:


```rust
let text = "{{ text | reversestr}}";
let globals = liquid::object!({
    "text": "Hello!",
});
let template = liquid::ParserBuilder::with_stdlib()
    .filter(ReverseStr)
    .build()
    .unwrap()
    .parse(text)
    .unwrap();
let output = template.render(&globals).unwrap();
assert_eq!(output, "!olleH".to_string());
```

