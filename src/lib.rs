use std::{fmt, collections::HashMap};

pub trait Tabular {
    fn to_table(&self) -> String;
}

impl<D: fmt::Display> Tabular for Vec<Vec<D>> {
    fn to_table(&self) -> String {
        return tabular(self)
    }
}

fn get_col_size_map(table_data: &Vec<Vec<impl fmt::Display>>) -> HashMap<usize, usize> {
    let mut col_size_map: HashMap<usize, usize> = HashMap::new();
    for row in table_data {
        for n in 0..row.len() {
            let col_size = row[n].to_string().len();
            if let Some(current_max) = col_size_map.get(&n) {
                if current_max < &col_size {
                    col_size_map.insert(n, col_size);
                }
            } else {
                col_size_map.insert(n, col_size);
            }
        }
    }
    return col_size_map;
}

fn pad_left(input: impl fmt::Display, length: &usize) -> String {
    let input_string = input.to_string();
    let remaining_space = length - input_string.len();
    let mut padded_string = " ".repeat(remaining_space);
    padded_string.push_str(&input_string);
    return padded_string; 
}

fn pad_right(input: impl fmt::Display, length: &usize) -> String {
    let mut input_string = input.to_string();
    let remaining_space = length - input_string.len();
    input_string.push_str(&" ".repeat(remaining_space));
    return input_string; 
}

fn tabular(table_data: &Vec<Vec<impl fmt::Display>>) -> String {
    let col_size_map = get_col_size_map(&table_data);
    let mut table: Vec<String> = vec![];

    let mut row: Vec<String> = vec![];
    for i in 0..table_data.len() {
        for j in 0..table_data[i].len() {
            let max_size = col_size_map.get(&j).unwrap();
            row.push(pad_right(&table_data[i][j], max_size));
        }
        table.push(row.join(" "));
        row = vec![];
    } 

    return table.join("\n");
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use super::*;

    struct CustomStruct<'a> {
        a: &'a str,
    }

    impl<'a> fmt::Display for CustomStruct<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.a)
        }
    }

    #[test]
    fn test_tabular_str() {
        let input_data = vec![vec!["A", "B", "C"], vec!["D", "E", "F"]];
        let expected = "A B C\nD E F";
        assert_eq!(input_data.to_table(), expected);
    }

    #[test]
    fn test_tabular_string() {
        let input_data = vec![
            vec![String::from("A"), String::from("B"), String::from("C")],
            vec![String::from("D"), String::from("E"), String::from("F")],
        ];
        let expected = "A B C\nD E F";
        assert_eq!(input_data.to_table(), expected);
    }

    #[test]
    fn test_tabular_number() {
        let input_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected = "1 2 3\n4 5 6";
        assert_eq!(input_data.to_table(), expected);
    }

    #[test]
    fn test_tabular_custom_struct() {
        let input_data = vec![
            vec![
                CustomStruct { a: "A" },
                CustomStruct { a: "B" },
                CustomStruct { a: "C" },
            ],
            vec![
                CustomStruct { a: "D" },
                CustomStruct { a: "E" },
                CustomStruct { a: "F" },
            ],
        ];
        let expected = "A B C\nD E F";
        assert_eq!(input_data.to_table(), expected);
    }

    #[test]
    fn test_tabular_max_col_size() {
        let input_data = vec![vec!["ABC", "B ", "C"], vec!["D", "EF", "F"]];
        let expected = "ABC B  C\nD   EF F";
        assert_eq!(input_data.to_table(), expected);
    }
}
