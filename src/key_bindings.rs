use std::collections::HashMap;

pub fn default_bindings<'a>() -> HashMap::<&'a str, usize>{
    let input_bindings: HashMap::<&'a str, usize> =
        HashMap::<&'a str, usize>::from([
            ("a1", 0),
            ("a2", 1),
            ("a3", 2),
            ("b1", 3),
            ("b2", 4),
            ("b3", 5),
            ("c1", 6),
            ("c2", 7),
            ("c3", 8),
        ]);
    return input_bindings;
}
