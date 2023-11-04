use std::{path::Path, collections::HashMap, fs, cmp, fmt::Display};

#[allow(dead_code)]
pub struct Rename {
    parent: Box<Path>,
    mapping: HashMap<String, String>,
}

impl Rename {
    fn new(parent: &Path, mapping: HashMap<String, String>) -> Rename {
        Rename { parent: parent.into(), mapping }
    }

    pub fn preview(parent: &Path) -> Rename {
        let mapping = generate_mapping(parent);
        Self::new(parent, mapping)
    }

    pub fn apply(&self) {
        for (old_name, new_name) in &self.mapping {
            fs::rename(self.parent.join(old_name), self.parent.join(new_name)).unwrap()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.mapping.is_empty()
    }
}

fn generate_mapping(parent: &Path) -> HashMap<String, String>{
    let mut mapping = HashMap::new();

    let mut old_name_to_len = HashMap::new();
    let mut digits = 0;
    
    let paths = fs::read_dir(parent).unwrap().map(|it|it.unwrap().path());
    for path in paths {
        if path.is_dir() {
            continue;
        }
        let stem = path.file_stem().unwrap().to_str().unwrap();
        // TODO: hexa and shits
        if stem.parse::<usize>().is_ok() {
            let len = stem.len();
            digits = cmp::max(digits, len);
            let old_name = path.file_name().unwrap().to_str().unwrap().to_string();
            old_name_to_len.insert(old_name, len);
        }
    }

    if digits <= 1 {
        return mapping;
    }

    // adding zeros
    for (old_name, len) in old_name_to_len {
        if len > digits {
            panic!("Something wrong with your program idiot.")
        }
        if len == digits {
            continue;
        }
        let zeros = digits - len;
        let mut new_name = String::with_capacity(old_name.len() + zeros);
        for _ in 0..zeros {
            new_name += "0";
        }
        new_name += &old_name;
        mapping.insert(old_name, new_name);
    }
    return mapping;
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