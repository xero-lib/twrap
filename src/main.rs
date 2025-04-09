use arboard::Clipboard;

fn main() {
    let mut cb = Clipboard::new().unwrap(); 

    let input = std::env::args().skip(1).collect::<Vec<String>>();
    let (data, remainder): (&[String; 2], &[String]) = input.split_first_chunk().expect("You didn't give me input bruv.");
    let message = remainder.join(" ");
    let (count, pattern) = (data[0].parse::<i16>().expect("Not a number."), data[1].clone());
    

    let output = match pattern.as_str() {
        "(" | ")" => wrap_it(count, message, "(", ")"),
        "[" | "]" => wrap_it(count, message, "[", "]"),
        "<" | ">" => wrap_it(count, message, "<", ">"),
        "{" | "}" => wrap_it(count, message, "{", "}"),
        x if x.starts_with("rev") => {
            let patt = x.split_once("rev").unwrap().1;
            wrap_it(count, message, patt, &reverse_it(patt))
        },
        x @ _ => wrap_it(count, message, x, x)
    };

    // let output = wrap_it(count, message, match pattern.as_str() {
    //     "(" | ")" => "(", ")" ,
    //     "[" | "]" => "[", "]",
    //     "<" | ">" => "<", ">",
    //     "{" | "}" =  "{", "}",
    //     x if x.starts_with("rev") => &x.split_once("rev").unwrap().1, &x.split_once("rev").unwrap().1.chars().rev().collect::<String>(),
    //     x @ _ => wrap_it(count, message, x, x)
    // };

    println!("{output}");

    cb.set_text(output).unwrap();
}

fn wrap_it(count: i16, mut message: String, start: &str, end: &str) -> String {
    for _ in 0..count {
        message.insert_str(0, &start);
        message.insert_str(message.len(), &end);
    }

    message
}

fn reverse_it(thing: &str) -> String {
    thing.chars().rev().map(|c| match c {
        '(' => ')',
        '[' => ']',
        '<' => '>',
        '{' => '}',
        ')' => '(',
        ']' => '[',
        '>' => '<',
        '}' => '{',
        _ => c,
    }).collect()
}
