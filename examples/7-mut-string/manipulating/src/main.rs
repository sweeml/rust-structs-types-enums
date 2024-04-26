fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    //println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    //println!("{}", description);

    // iterate over the characters in the sentence
    // for c in sentence.chars() {
    //     match c {
    //         'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
    //         _ => continue,
    //     }
    // }
    let mut vowel_counts: [u32; 5] = [0; 5];
    for c in sentence.chars() {
        match c {
            'a' => vowel_counts[0] += 1,
            'e' => vowel_counts[1] += 1,
            'i' => vowel_counts[2] += 1,
            'o' => vowel_counts[3] += 1,
            'u' => vowel_counts[4] += 1,
            _ => continue,
        }
    }
    println!("Number of occurrences of 'a': {}", vowel_counts[0]);
    println!("Number of occurrences of 'e': {}", vowel_counts[1]);
    println!("Number of occurrences of 'i': {}", vowel_counts[2]);
    println!("Number of occurrences of 'o': {}", vowel_counts[3]);
    println!("Number of occurrences of 'u': {}", vowel_counts[4]);
    // Split and collect into a vector
    let words: Vec<&str> = sentence.split_whitespace().collect();
    // let words = sentence.split(' ').collect::<Vec<&str>>();
    // //println!("{:?}", words);

    // let reversed = sentence.chars().rev().collect::<String>();
    // println!("{}", reversed);
fn longest_word(sentence: &str) -> &str {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let mut max_length = 0;
    let mut max_word = "";

    for word in words {
        if word.len() > max_length {
            max_length = word.len();
            max_word = word;
        }
    }

    max_word
}

let sentence = "This is a test sentence";
let longest = longest_word(sentence);
println!("The longest word is: {}", longest);
    
}
