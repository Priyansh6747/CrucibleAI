use std::io::{stdin, stdout};
use crossterm::{
    style::{Color, SetForegroundColor,ResetColor },
    ExecutableCommand
};
pub fn get_user_response(question: &str) -> String {
    let mut stdout = stdout();

    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("\n{}", question);
    
    stdout.execute(ResetColor).unwrap();
    
    let mut res = String::new();
    stdin().read_line(&mut res).expect("Did not enter a correct string");
    
    res.trim().to_string()
}