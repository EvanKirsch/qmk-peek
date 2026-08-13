use regex::Regex;
use std::collections::BTreeMap;

pub fn split_args(inner: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    for ch in inner.chars() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        if ch == ',' && depth == 0 {
            let trimmed = cur.trim().to_string();
            args.push(trimmed);
            cur = String::new();
        } else {
            cur.push(ch);
        }
    }
    if !cur.trim().is_empty() {
        args.push(cur.trim().to_string());
    }
    args
}

pub fn detect_macro<'a>(src: &str, known_macros: &[&'a str]) -> Option<&'a str> {
    for &macro_name in known_macros {
        let pattern = format!(r"\b{}\s*\(", regex::escape(macro_name));
        let re = Regex::new(&pattern).unwrap();
        if re.is_match(src) {
            return Some(macro_name);
        }
    }
    None
}

pub fn extract_layers(src: &str, macro_name: &str) -> BTreeMap<usize, Vec<String>> {
    let mut layers = BTreeMap::new();
    let pattern = format!(r"\[(\d+)\]\s*=\s*{}\(", regex::escape(macro_name));
    let re = Regex::new(&pattern).unwrap();
    let bytes = src.as_bytes();

    for caps in re.captures_iter(src) {
        let idx: usize = caps[1].parse().unwrap();
        let m = caps.get(0).unwrap();
        let start = m.end();
        let mut depth = 1i32;
        let mut i = start;
        while depth > 0 {
            match bytes[i] {
                b'(' => depth += 1,
                b')' => depth -= 1,
                _ => {}
            }
            i += 1;
        }
        let inner = &src[start..i - 1];
        layers.insert(idx, split_args(inner));
    }
    layers
}
