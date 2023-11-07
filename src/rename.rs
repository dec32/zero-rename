use std::{path::Path, collections::{HashMap, BTreeMap}, fs, cmp, fmt::Display};
use crate::errors::Result;

#[allow(dead_code)]
type Mapping = BTreeMap<String, String>;

pub struct Rename {
    parent: Box<Path>,
    mapping: Mapping,
}

impl Rename {
    fn new(parent: &Path, mapping: Mapping) -> Self {
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

    pub fn mapping(&self) -> &Mapping {
        &self.mapping
    }

    pub fn parent(&self) -> &Path {
        &self.parent
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


impl Display for Rename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Rename {
    fn to_string(&self) -> String {
        let key_col = "Original";
        let val_col = "Renamed To";
        let key_width = cmp::max(key_col.len(), self.mapping.keys().map(|it|it.len()).max().unwrap_or(0));
        let val_width = cmp::max(val_col.len(), self.mapping.values().map(|it|it.len()).max().unwrap_or(0));

        let line_len = "| ".len() + key_width + " | ".len() + val_width + " |\n".len();
        let estimated_len = line_len * (2 + self.mapping.len());

        let mut buf = String::with_capacity(estimated_len);
        let s = &mut buf;

        append(s, "| ");
        fill_up(s, key_col, key_width);
        append(s, " | ");
        fill_up(s, val_col, val_width);
        append(s, " |");
        new_line(s);
    
        append(s, "|-");
        fill_with(s, '-',key_width);
        append(s, "-|-");
        fill_with(s, '-',val_width);
        append(s, "-|");
        new_line(s);
    
        // body
        for (key, val) in &self.mapping {
            append(s, "| ");
            fill_up(s, &key, key_width);
            append(s, " | ");
            fill_up(s, &val, val_width);
            append(s, " |");
            new_line(s);
        }
        buf
    }
}


// a few functions to make things easier
#[inline]
fn append(s: &mut String, text: &str) {
    *s += text;
}

#[inline]
fn new_line(s: &mut String) {
    s.push('\n')
}

#[inline]
fn fill_up(s: &mut String, text: &str, len: usize) {
    append(s, text);
    for _ in 0..len-text.len() {
        s.push(' ');
    }
}

#[inline]
fn fill_with(s: &mut String, filler: char, len: usize) {
    for _ in 0..len {
        s.push(filler);
    }
}