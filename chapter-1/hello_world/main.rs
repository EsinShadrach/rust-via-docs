fn main() {
    let pad_fruit = left_pad("apple", '*', 4);
    let fib = fib(3);

    println!("fib: {}", fib);
}

fn left_pad(word: &str, character: char, count: u32) -> String {
    let mut result = String::new();

    for _ in 0..count {
        result.push_str(&character.to_string());
    }

    result.push_str(&word);

    return result;
}

fn right_pad(word: &str, character: char, count: u32) -> String {
    let mut result = String::new();

    result.push_str(&word);

    for _ in 0..count {
        result.push_str(&character.to_string());
    }

    return result;
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    let mut prev = 0;
    let mut current = 1;

    for _ in 2..=n {
        let next = current + prev;
        prev = current;
        current = next;
    }

    return current;
}
