pub mod pig_latin {

    fn is_an_vowel(current_char: char) -> bool {
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];
        for vowel in vowels {
            if vowel == current_char {
                return true;
            }
        }
        return false;
    }

    pub fn to_pig_latin(text: String) -> String {
        let first_char = text.chars().nth(0).unwrap();

        if is_an_vowel(first_char) {
            return format!("{}-hey", text);
        } else {
            let mut without_first = text.chars();
            without_first.next();
            return format!("{}-{}ay", without_first.as_str(), first_char);
        }
    }
}
