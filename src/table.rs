use std::{collections::HashMap, cmp};

#[inline]
pub fn print(map: &HashMap<String, String>, key_col: &str, val_col: &str) {
    let key_width = cmp::max(key_col.len(), map.keys().map(|it|it.len()).max().unwrap_or(0));
    let val_width = cmp::max(val_col.len(), map.values().map(|it|it.len()).max().unwrap_or(0));

    // head
    print!("| ");
    print_fill(key_col, key_width);
    print!(" | ");
    print_fill(val_col, val_width);
    print!(" |");
    println();

    print!("|-");
    fill('-',key_width);
    print!("-|-");
    fill('-',val_width);
    print!("-|");
    println();

    // body
    for (key, val) in map {
        print!("| ");
        print_fill(key, key_width);
        print!(" | ");
        print_fill(&val, val_width);
        print!(" |");
        println();
    }

    #[inline]
    fn print_fill(content: &str, len:usize) {
        print!("{}", content);
        fill(' ', len-content.len());
    }

    #[inline]
    fn fill(filler:char, len:usize) {
        for _ in 0..len {
            print!("{}", filler);
        }
    }

    #[inline]
    fn println() {
        print!("\n")
    }
}