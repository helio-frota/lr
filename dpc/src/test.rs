#[cfg(test)]
mod tests {
    use crate::util;
    use std::fs;

    fn get_code_blocks() -> Vec<String> {
        let files = util::rust_files(".", "duplicrabs").expect("rust files not found.");
        let path_as_str = files[0].to_string_lossy();
        let content = fs::read_to_string(path_as_str.to_string());
        util::code_blocks(content.unwrap().as_str())
    }

    #[test]
    fn test_block_counter() {
        let blocks = get_code_blocks();
        for cb in &blocks {
            let f = util::block_counter(cb);
            assert!(f.0 > 0);
            assert!(f.1 > 0);
        }
    }

    #[test]
    fn test_similar() {
        let blocks = get_code_blocks();
        // Intentionally lowering the threshold to test something
        let similar_found = util::similar(blocks, 0.10);
        assert!(!similar_found.is_empty());
        for s in &similar_found {
            assert!(!s.0.is_empty());
            assert!(!s.1.is_empty());
            assert!(s.2 > 0.10);
        }
    }

    #[test]
    fn test_report() {
        let blocks = get_code_blocks();
        // Intentionally lowering the threshold to test something
        let similar_found = util::similar(blocks, 0.10);
        let out = util::report(similar_found);
        assert!(out.contains("Duplicrabs"));
        assert!(out.contains("Almost the same"));
    }
}
