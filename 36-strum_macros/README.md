An example of using `strum_macros`.

## How to use `strum_macros` crate

- EnumString
- EnumCount
- EnumIter
- EnumMessage
- EnumIs
- EnumDiscriminants
- AsRefStr
- Display
- ...

## Use `cargo expand` to see the generated code

The cargo expand command can be used to show the generated code.

```bash
cargo expand [module_name]
```

The following code:

```rust
#[derive(EnumString)]
enum Color {
    Red,
    Green {
        range: usize,
    },
    #[strum(serialize = "blue", serialize = "b")]
    Blue,
    #[strum(disabled)]
    Yellow,
    #[strum(ascii_case_insensitive)]
    Black,
}
```

will be expanded to something like:

```rust
impl ::core::str::FromStr for Color {
    type Err = ::strum::ParseError;
    fn from_str(
        s: &str,
    ) -> ::core::result::Result<Color, <Self as ::core::str::FromStr>::Err> {
        ::core::result::Result::Ok(
            match s {
                "Red" => Color::Red,
                "Green" => {
                    Color::Green {
                        range: Default::default(),
                    }
                }
                "blue" => Color::Blue,
                "b" => Color::Blue,
                s if s.eq_ignore_ascii_case("Black") => Color::Black,
                _ => {
                    return ::core::result::Result::Err(
                        ::strum::ParseError::VariantNotFound,
                    );
                }
            },
        )
    }
}
```
