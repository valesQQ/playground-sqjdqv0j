### You will find below some solutions for the exercise


It is possible to find the solution by iterating the string the same way as you would likely do in Python or C/C++.
This gives a quite long answer:
```
pub fn decipher_message(morse_message: &str) -> String {
    let mut answer = String::new();
    let mut start = 0;

    for (i, c) in morse_message.chars().enumerate() {
        if (c == ' ') || (i == morse_message.len()-1) {
            
            // Special handle for last char
            if i == (morse_message.len() - 1) {
                println!("{}", &morse_message[start..i + 1]);
                answer.push_str(
                    _morse_to_alphanumeric_dictionary()
                        .get(&morse_message[start..i + 1])
                        .expect("Wrong"),
                );
            } else {
                println!("{}", &morse_message[start..i]);
                answer.push_str(
                    _morse_to_alphanumeric_dictionary()
                        .get(&morse_message[start..i])
                        .expect("Wrong"),
                );
            }
            // Move start at start of next char
            start = i + 1;
        }
    }

    return answer;
}
```

However, Rust gives us the dedicated tools to solve this problem in a one-liner.
```
pub fn decipher_message(morse_message: &str) -> String {
morse_message.split(' ')
    .map(|letter|{*_morse_to_alphanumeric_dictionary()
    .get(letter)
    .unwrap()}).collect::<Vec<_>>().join("")
}
```