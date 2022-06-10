use tabular::Tabular;

fn main() {
    let table_data = vec![
        vec!["Cat", "Dog", "Elephant"],
        vec!["Pidgeon", "Bear", "Wolf"]
    ];

    println!("{}", table_data.to_table());
}