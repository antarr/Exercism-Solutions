
extern crate unicode_segmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
