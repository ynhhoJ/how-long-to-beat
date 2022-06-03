pub fn is_even_number(number: usize) -> bool {
    let is_even = number % 2;

    if is_even == 1 {
        return false;
    }

    return true;
}
