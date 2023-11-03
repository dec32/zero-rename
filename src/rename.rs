use std::{path::Path, fs, cmp, collections::HashMap};
#[allow(dead_code)]
pub struct Rename {
    parent: Box<Path>,
    mapping: HashMap<String, String>,
}

// todo: this name is so stupid
pub fn generate(parent: &Path) -> HashMap<String, String>{
    let mut mapping = HashMap::new();

    let mut old_name_to_len = HashMap::new();
    let mut digits = 0;

    for entry in fs::read_dir(parent).unwrap() {
        let path = entry.unwrap().path();
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
        let mut new_name = String::new();
        for _i in 0..zeros {
            new_name += "0";
        }
        new_name += &old_name;
        mapping.insert(old_name, new_name);
    }
    return mapping;
}



pub fn apply(parent: &Path, mapping: HashMap<String, String>) {
    for (old_name, new_name) in mapping {
        fs::rename(parent.join(old_name), parent.join(new_name)).unwrap()
    }
}


