use std::collections::HashMap;

lazy_static! {
    pub static ref ARROW: HashMap<&'static str, &'static str> = {
        return [
            ("DoubleUp", "⬆⬆"),
            ("SingleUp", "⬆"),
            ("FortyFiveUp", "↗"),
            ("Flat", "➡"),
            ("FortyFiveDown", "↘"),
            ("SingleDown", "⬇"),
            ("DoubleDown", "⬇⬇"),
        ].iter().cloned().collect();
    };
}


#[test]
fn test_arrow() {
    println!(" arrow: {}", ARROW.get(&"Norway").unwrap())
}