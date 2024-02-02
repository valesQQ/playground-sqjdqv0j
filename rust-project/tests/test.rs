extern crate rust_template;


use rust_template::decipher_message;

#[test]
pub fn test_universe() {
    let expected: &str = "SOS ENDOR";
    let morse_message: &str = "... --- ... / . -. -.. --- .-.";
    let actual = decipher_message(morse_message);
    let res = (expected == actual);
    assert!(res, "Wrong!");
}


