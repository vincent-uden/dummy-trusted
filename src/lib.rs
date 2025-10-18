use std::path::Path;

use mupdf::Document;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn sub(left: u64, right: u64) -> u64 {
    left - right
}

pub fn open_pdf(path: &Path) {
    let doc = Document::open(path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
