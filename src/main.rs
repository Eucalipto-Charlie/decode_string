use std::io::{self};

fn decode_string(s: &str) -> String {
    let mut num_stack: Vec<usize> = Vec::new();
    let mut str_stack: Vec<String> = Vec::new();
    let mut curr = String::new();
    let mut num = 0;

    for c in s.chars() {
        if c.is_ascii_digit() {
            num = num * 10 + c.to_digit(10).unwrap() as usize;
        } else if c == '[' {
            num_stack.push(num);
            str_stack.push(curr.clone());
            curr.clear();
            num = 0;
        } else if c == ']' {
            let repeat = num_stack.pop().unwrap();
            let mut temp = str_stack.pop().unwrap();
            for _ in 0..repeat {
                temp.push_str(&curr);
            }
            curr = temp;
        } else {
            curr.push(c);
        }
    }

    curr
}

fn main() {
    println!("请输入编码字符串：");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let decoded = decode_string(input);
    println!("解码结果：{}", decoded);
}
