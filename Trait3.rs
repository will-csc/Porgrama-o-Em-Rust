fn main(){
    trait LettersCount {
        fn letters_count(&self, ch: char) -> usize;
    }
    impl LettersCount for str {
        fn letters_count(&self, ch: char) -> usize {
            self.chars().filter(|c| *c == ch).count()
        }
    }
    print!("{}\n", "".letters_count('a'));
    print!("{}\n", "ddd".letters_count('a'));
    print!("{}\n", "ddd".letters_count('d'));
    print!("{}\n", "foobarbaz".letters_count('a'));
}