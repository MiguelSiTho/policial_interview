use std::{io, vec::Vec, string::String};

fn main() {
    println!("This program is a policial interview simulator, we will ask some question and you will have to answer.\n\nPlease type the suspect name:");
    let mut suspect = String::new();
    io::stdin().read_line(&mut suspect).expect("Error reading");

    let questions = vec![
        "The first question is: Does the suspect live near the victim?",
        "The second question is: Has the suspect ever worked with the victim?",
        "The third question is: Did the suspect call the victim?", 
        "The fourth question is: Was the suspect at the crime scene?",
        "The fifth question is: Did the suspect owe the victim?"
    ];

    let mut k = 0;
    let mut answers = vec![String::new(), String::new(), String::new(), String::new(), String::new()];
    for i in questions.iter() {
        println!("\n{}", i);
        for _j in &answers {
            input(&mut answers, k);
            k += 1;
            break;
        }
    }
    
    let mut n: usize = 0;
    let mut points = 0;
    for _i in &answers {
        match &answers[n].to_uppercase() as &str {
            "YES" => points += 1,
            "NO" => points += 0,
            _ => println!("Error")
        }
        n += 1;
    }

    match points {
        5 => println!("{} is the assasin.", suspect.trim()),
        4 | 3 => println!(r"{} is accomplice.", suspect.trim()),
        2 => println!(r"{} is still a suspect.", suspect.trim()),
        _ => println!(r"{} is innocent.", suspect.trim())
    }
}
fn input(input: &mut Vec<String>, n: i32) {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Error reading");
    input[n as usize] = a.trim().to_string();
}