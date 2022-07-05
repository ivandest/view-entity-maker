pub fn make_camel_case(snake_string: String, upper_first: bool) -> String {
    let input_vector: Vec<char> = snake_string.chars().collect();

    let mut output: Vec<char> = Vec::new();
    let mut index = 0;
    let mut is_delimiter = false;

    for x in input_vector {
        if upper_first == true && index == 0 {
            output.push(x.to_ascii_uppercase());
            index = index + 1;

            continue;
        }

        if x == '_' {
            is_delimiter = true;

            continue;
        }

        if is_delimiter == true {
            output.push(x.to_ascii_uppercase());
            is_delimiter = false;

            continue;
        }

        output.push(x.to_ascii_lowercase());
        index = index + 1;
    }

    output.iter().collect()
}

#[test]
fn check_camel_case() {
    assert_eq!(
        make_camel_case(String::from("test_string"), false),
        "testString"
    );
    assert_eq!(
        make_camel_case(String::from("test_string"), true),
        "TestString"
    );
}

pub fn make_kebab_case(snake_string: String) -> String {
    snake_string.replace("_", "-")
}

#[test]
fn check_kebab_case() {
    assert_eq!(make_kebab_case(String::from("test_string")), "test-string");
}
