fn number_to_word(n: u32) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        _ => "",
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn bottles_str(n: u32) -> String {
    match n {
        0 => "no green bottles".to_string(),
        1 => "1 green bottle".replace('1', "one"),
        _ => format!("{} green bottles", number_to_word(n)),
    }
}

pub fn recite_verse(start: u32) -> String {
    let current_word = capitalize(number_to_word(start));
    let current_noun = if start == 1 { "bottle" } else { "bottles" };
    
    let next_bottles = bottles_str(start - 1);

    format!(
        "{current_word} green {current_noun} hanging on the wall,\n\
         {current_word} green {current_noun} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {next_bottles} hanging on the wall."
    )
}

pub fn recite(start: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|i| recite_verse(start - i))
        .collect::<Vec<_>>()
        .join("\n\n")
}