# tabular

A simple table trait object for `Vec<Vec<impl fmt::Display>>`

```rust
use tabular::Tabular;

fn main() {
    let table_data = vec![
        vec!["Cat", "Dog", "Elephant"],
        vec!["Pidgeon", "Bear", "Wolf"]
    ];

    println!("{}", table_data.to_table());
}
```
```bash
> cargo run

Cat     Dog  Elephant
Pidgeon Bear Wolf  
```