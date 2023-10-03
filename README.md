# Liquid filter for Rust to reverse a string

The [liquid](https://crates.io/crates/liquid) crate, the Rust implementation of the [liquid](https://shopify.github.io/liquid/) template
system has many filters to manipulate the data in the template, but AFAIK there is no filter to reverse a string.

This crate was originally developed as a demo on how to add a filter to liquid.

## Usage:

* `Cargo.toml`:

```toml
[dependencies]
liquid = "0.26"
liquid-filter-reverse-string = "0.1"
```

* `src/main.rs`:


```rust
use liquid_filter_reverse_string::ReverseStr;

fn main() {
    let template = "reversed: {{text | reversestr}}";
    let text = "Hello World!";

    let result = render(&template, &text);
    println!("{}", result);
    assert_eq!(result, "reversed: !dlroW olleH");
}


fn render(tmpl: &str, text: &str) -> String {
    let globals = liquid::object!({
        "text": text,
    });

    let template = liquid::ParserBuilder::with_stdlib()
        .filter(ReverseStr)
        .build()
        .unwrap()
        .parse(tmpl).unwrap();

    let output = template.render(&globals).unwrap();
    return output
}
```

The important pieces:

The `use` statement:

```
use liquid_filter_reverse_string::ReverseStr;
```

The use of the `reversestr` filter in the template

```
let template = "reversed: {{text | reversestr}}";
```

* adding the filter to the engine:

```rust
  .filter(ReverseStr)
```



## Release


* update the `version` number in `Cargo.toml`

```
cargo publish
git tag -a v0.1.0 -m v0.1.0
git push --tags
```
