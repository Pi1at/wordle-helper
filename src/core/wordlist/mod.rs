pub mod pattern;
const RU5WORDS: &str = include_str!("./assets/ru_5words.txt");
const CHARACTERS: &str = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя";
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let words = dbg!(RU5WORDS.split('\n').collect::<Vec<&str>>());
    }
}
