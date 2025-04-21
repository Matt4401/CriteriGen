use regex::Regex;

pub fn looks_like_c_function(input: &str) -> bool {
    let function_pattern = Regex::new(
        r"(?m)^\s*(int|void|char|float|double|long|short|unsigned|signed|bool)\s+\w+\s*\([^)]*\)\s*\{"
    ).unwrap();

    function_pattern.is_match(input)
}
