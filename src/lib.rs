fn div_by_three(num: usize) -> bool {
    num % 3 == 0
}

fn div_by_five(num: usize) -> bool {
    num % 5 == 0
}

fn div_by_fifteen(num: usize) -> bool {
    div_by_three(num) && div_by_five(num)
}

fn main() {
    for num in (1..101) {
        println!("{}",
            if div_by_fifteen(num) { "FizzBuzz" }
            else if div_by_three(num) { "Fizz" }
            else if div_by_five(num) { "Buzz" }
            else { "" }
        );
    }
}

#[test]
fn test_div_by_three() {
    if div_by_three(1) {
        panic!("One is not three");
    }
}

#[test]
fn test_div_by_three_with_three() {
    assert!(div_by_three(3), "Three should be three");
}

#[test]
fn test_div_by_five() {
    if div_by_five(3) {
        panic!("Three is not five");
    }
}

#[test]
fn test_div_by_five_with_five() {
    assert!(div_by_five(5), "Five should be five");
}

#[test]
fn test_div_by_fifteen() {
    if div_by_fifteen(5) {
        panic!("Five is not fifteen");
    }
}

#[test]
fn test_div_by_fifteen_with_fifteen() {
    assert!(div_by_fifteen(15), "Fifteen should be fifteen");
}
