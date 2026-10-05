use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;

struct Root<'a> {
    content: Vec<Word<'a>>,
}
impl<'a> Root<'a> {
    fn find(&self, word: &str) -> Option<usize> {
        let mut found = false;
        let mut i = 0;
        let length = self.content.len();
        
        while !found && i < length {
            let mut node = &self.content[i];
            if node.equals(word) {
                found = true;
            } else {
                i +=1;
            }
        }
        match found {
            false => None,
            true => Some(i),
        }
    }
    fn add_string<I>(&mut self, mut words: I)
    where
        I : Iterator<Item = &'a str>,
    {
        let word = match words.next() {
            Some(x) => x,
            None => return,
        };
        
        match self.find(word) {
            Some(x) => self.content[x].increment(words),
            None => {
                let mut new_word = Word {value: word, count: 0, postwords: Vec::new()};
                new_word.increment(words);
                self.content.push(new_word);
            },
        }
    }
    fn most_likely(&self) -> Option<&str> {
        if self.content.len() == 0 {
            return None;
        }
        let mut most_likely: usize = 0;
        for i in 1..self.content.len() {
            if self.content[i].count() > self.content[most_likely].count() {
                most_likely = i;
            }
        }
        Some(self.content[most_likely].value())
    }
    fn get_next<'q, I>(&self, mut words: I) -> Option<&str>
    where
        I : Iterator<Item = &'q str>,
    {
        let word = match words.next() {
            Some(x) => x,
            None => return self.most_likely(),
        };
        match self.find(word) {
            Some(x) => self.content[x].get_next(words),
            None => None,
        }
    }
    fn print(&self) {
        println!("Top:");
        for word in &self.content {
            print!("  ");
            word.print();
        }
    }
}
struct Word<'a> {
    value: &'a str,
    count: u32,
    postwords: Vec<Word<'a>>,
}
impl<'a> Word<'a> {
    fn most_likely(&self) -> Option<&str> {
        if self.postwords.len() == 0 {
            return None;
        }
        let mut most_likely: usize = 0;
        for i in 1..self.postwords.len() {
            if self.postwords[i].count() > self.postwords[most_likely].count() {
                most_likely = i;
            }
        }
        Some(self.postwords[most_likely].value())
    }
    fn find(&self, word: &str) -> Option<usize> {
        let mut found = false;
        let mut i = 0;
        let length = self.postwords.len();
        
        while !found && i < length {
            let mut node = &self.postwords[i];
            if node.equals(word) {
                found = true;
            } else {
                i +=1;
            }
        }
        match found {
            false => None,
            true => Some(i),
        }
    }
    fn count(&self) -> u32 {
        self.count
    }
    fn value(&self) -> &str {
        self.value.clone()
    }
    fn equals(&self, word: &str) -> bool {
        word == self.value
    }
    fn increment<I>(&mut self, mut words: I) 
    where
        I : Iterator<Item = &'a str>,
    {
        self.count += 1;
        let word = match words.next() {
            Some(x) => x,
            None => return,
        };

        let index = self.find(word);
        match index {
            Some(x) => self.postwords[x].increment(words),
            None => {
                let mut new_word = Word {value: word, count: 0, postwords: Vec::new()};
                new_word.increment(words);
                self.postwords.push(new_word);
            },
        }
    }
    fn get_next<'q, I>(&self, mut words: I) -> Option<&str>
    where
        I : Iterator<Item = &'q str>,
    {
        let word = match words.next() {
            Some(x) => x,
            None => return self.most_likely(),
        };
        match self.find(word) {
            Some(x) => self.postwords[x].get_next(words),
            None => None,
        }
    }
    fn print(&self) {
        println!("{}:", self.value);
        for word in &self.postwords {
            print!("    ");
            word.print();
        }
    }
}

fn process_text(text: String) -> String{
    let mut processing = text.chars().map(|x| if x.is_alphabetic() {x} else {' '}).collect::<String>();
    processing.make_ascii_lowercase();
    processing
}
fn main() {
    let raw_text= fs::read_to_string(Path::new("./foo.txt")).expect("Should have been able to read the file");
    let mut clean_text = process_text(raw_text);
    let mut word_buffer: VecDeque<&str> = clean_text.split_ascii_whitespace().collect::<VecDeque<&str>>();
    let mut tree = Root { content: Vec::new(), };
    for i in 0..(word_buffer.len()) {
        tree.add_string(word_buffer.clone().into_iter());
        word_buffer.pop_front();
    }
    let mut test_input = String::from("");
    let mut valid = true;
    while valid {
        let buffer = test_input.clone();
        match tree.get_next(buffer.split_ascii_whitespace()) {
            Some(x) => {
                test_input.push_str(" ");
                test_input.push_str(x);
            },
            None => valid = false,
        }
    }
    println!("{}", test_input);
}

