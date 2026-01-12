pub fn convert_to_pig_latin(sentence: &String) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut result: Vec<String> = Vec::new();

    for word in sentence.split_whitespace() {
        let chars: Vec<char> = word.chars().collect();

        for (index, letter) in word.char_indices() {
            if vowels.contains(&letter.to_ascii_lowercase()) {
                if index == 0 {
                    result.push(format!("{word}-hay"));
                    break;
                } else {
                    let head: String = chars[..index].iter().collect();
                    let tail: String = chars[index..].iter().collect();
                    result.push(format!("{tail}-{head}ay"));
                    break;
                }
            }

            if index == chars.len() - 1 {
                result.push(word.to_string());
            }
        }
    }

    result.join(" ")
}
