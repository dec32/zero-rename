use std::{path::Path, collections::HashMap, fs, cmp, fmt::Display};
use crate::errors::Result;

#[allow(dead_code)]
type Mapping = HashMap<String, String>;

pub struct Rename {
    parent: Box<Path>,
    mapping: Mapping,
}

impl Rename {
    fn new(parent: &Path, mapping: HashMap<String, String>) -> Self {
        Rename { parent: parent.into(), mapping }
    }

    pub fn preview(parent: &Path) -> Result<Self> {
        let mapping = generate_mapping(parent)?;
        Ok(Self::new(parent, mapping))
    }

    #[allow(dead_code)]
    pub fn rename(parent: &Path) -> Result<Self>{
        let rename = Self::preview(parent)?;
        rename.apply()?;
        Ok(rename)
    }

    pub fn apply(&self) -> Result<()>{
        for (old_name, new_name) in &self.mapping {
            fs::rename(self.parent.join(old_name), self.parent.join(new_name))?
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.mapping.is_empty()
    }
}

fn generate_mapping(parent: &Path) -> Result<Mapping>{
    let mut mapping = Mapping::new();

    let mut name_to_digits = HashMap::new();
    let mut max_digits = 0;
    
    let entries = fs::read_dir(parent)?;
    for entry in entries {
        let path = entry?.path();
        if path.is_dir() {
            continue;
        }
        let Some(stem) = path.file_stem() else {
            // todo logging
            continue;
        };
        let Some(stem) = stem.to_str() else {
            // todo logging
            continue;
        };
        if is_decimal_digits(stem) {
            let digits = stem.len();
            max_digits = cmp::max(max_digits, digits);
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            name_to_digits.insert(name, digits);
        }
    }

    if max_digits <= 1 {
        return Result::Ok(mapping);
    }

    // adding zeros
    for (name, digits) in name_to_digits {
        if digits > max_digits {
            panic!("Something wrong with your program idiot.")
        }
        if digits == max_digits {
            continue;
        }
        let zeros = max_digits - digits;
        let mut new_name = String::with_capacity(name.len() + zeros);
        for _ in 0..zeros {
            new_name += "0";
        }
        new_name += &name;
        mapping.insert(name, new_name);
    }
    return Result::Ok(mapping);
}


fn is_decimal_digits (text: &str) -> bool{
    for ch in text.chars() {
        if ch < '0' || ch > '9' {
            return false;
        }
    }
    return true;
}

impl Rename {
    pub fn print(&self) {
        let key_col = "Original";
        let val_col = "New";

        let key_width = cmp::max(key_col.len(), self.mapping.keys().map(|it|it.len()).max().unwrap_or(0));
        let val_width = cmp::max(val_col.len(), self.mapping.values().map(|it|it.len()).max().unwrap_or(0));
    
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
        for (key, val) in &self.mapping {
            print!("| ");
            print_fill(&key, key_width);
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
}

// how the hell should i implement this thing?
impl Display for Rename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}