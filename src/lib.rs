use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_add() {
        assert_eq!(add(1, 2), 13);
        assert_eq!(add(10, 20), 30);
    }
}
  