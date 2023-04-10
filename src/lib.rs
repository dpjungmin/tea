mod app;
mod compositor;
mod stats;

pub use app::App;

pub const EXAMPLE_TEXT: &str = r#"pub fn add(left: usize, right: usize) -> usize {
    left + right
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

// Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur lacinia nibh id mi pellentesque finibus. Etiam suscipit, felis sed laoreet venenatis, sem quam efficitur purus, vitae commodo nisl.
// Please note that the text provided is a commonly used placeholder text called "Lorem Ipsum." It is used to fill in spaces where actual text will be placed later on.
"#;
