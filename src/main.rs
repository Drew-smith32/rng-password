use rand::*;

fn main() {
    let mut characters = Vec::<char>::new();

    for s in 'a'..='z' {
        characters.push(s);
    }

    for s in 'A'..='Z' {
        characters.push(s);
    }
    
    for s in '1'..='9' {
        characters.push(s);
    }

    let mut password = String::new();
    let length = 15;

    let mut rng = thread_rng();
   
    for _i in 0..length {
    let random_index = rng.gen_range(0..characters.len()); 
     password.push(characters[random_index]);
    
    }

    println!("{}",password);
}
