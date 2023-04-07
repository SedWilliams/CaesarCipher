use std::env;

struct Lists {
    list: Vec<char>,
    shifted_list: Vec<char>,
    output_list: String,
}

impl Lists {
    fn new() -> Lists {
        let lists: Lists = Lists { 
             list: vec!['A', 'B', 'C', 'D',
                'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
                'W', 'X', 'Y', 'Z'
            ],
            shifted_list: vec!['A', 'B', 'C', 'D',
                'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
                'W', 'X', 'Y', 'Z'
            ],
            output_list: String::new(),
        };

        lists
    }
}

struct Config {
    num: i32,
    message: String,
}

impl Config {
    fn parse_config() -> Config {
        let args: Vec<String> = env::args().collect();
        let config: Config = Config { 
            num: args[1].clone().trim().parse().expect("NaN foo"), 
            message: args[2].clone(), 
        };
        config
    }
}

fn shift(shifts: i32, shifted_list: &mut Lists) {
    for elements in 0..shifts {
        let moved_letter = shifted_list.shifted_list.remove(0);
        shifted_list.shifted_list.push(moved_letter);
    }
   // println!("{:?}", shifted_list.shifted_list);
}

fn return_letter_index(letter: char, alphabet: &mut Lists) -> usize {
    let upper_letter = letter.to_ascii_uppercase();
    let mut index = 0;
    for (i, val) in alphabet.list.iter().enumerate() {
        if *val == upper_letter {
            index = i;
            break;
        }
    }
    return index;
}


fn append_to_output(letter: char, lists: &mut Lists) {
    let index_in_alphabet = return_letter_index(letter, lists);
    //let encoded_character = lists.shifted_list[index_in_alphabet];
    let encoded_character = lists.shifted_list.get(index_in_alphabet).unwrap();
    lists.output_list.push(*encoded_character);
}

fn encode(msg: String, output_list: &mut Lists) {
    for i in msg.chars() {
        match i {
            ' ' => output_list.output_list.push(' '),
            _ => append_to_output(i, output_list),
        }
    }

    println!("Message encoded!: {}", output_list.output_list);
}

fn main() {
    
    let mut lists: Lists = Lists::new();
    let config: Config = Config::parse_config();
    shift(config.num, &mut lists);
    encode(config.message, &mut lists);
    
}















