use std::env;
use std::fs;
    #[derive(Clone, Debug)]
    pub struct God{
        words:Vec<String>,
        amount:usize
    }
    impl God{
        pub fn init(amount: usize) -> Self {
            Self {
                words: Self::getWords("words.txt"),
                amount,
            }
        }
        pub fn getWords(path:&str)->Vec<String>{
            

            let contents = fs::read_to_string(path)
                .expect("Oops");
            let words = contents.lines();
            
            let vec = words.map(str::to_string).collect::<Vec<String>>();
            /*for word in &vec{
                println!("{}",word)
            }*/
            vec
        } 
        pub fn speak(&self)->String{
            (0..self.amount)
            .map(|_| self.words[fastrand::usize(..self.words.len())].to_string())
            .collect::<Vec<String>>()
            .join(" ")
            }
        }
    
