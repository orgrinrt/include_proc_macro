pub fn fizzbuzz(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_str = input.to_string();
    let n = input_str.trim().parse::<i32>().unwrap_or(0);

    let result = if n % 15 == 0 {
        "\"FizzBuzz\"".to_string()
    } else if n % 3 == 0 {
        "\"Fizz\"".to_string()
    } else if n % 5 == 0 {
        "\"Buzz\"".to_string()
    } else {
        format!("{}", n)
    };

    result.parse().unwrap()
}
