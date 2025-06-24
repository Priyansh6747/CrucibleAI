use std::io::{stdin, stdout};
use crossterm::{
    style::{Color, SetForegroundColor,ResetColor },
    ExecutableCommand
};
use crossterm::style::Print;

pub fn get_user_response(question: &str) -> String {
    let mut stdout = stdout();

    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("\n{}", question);
    
    stdout.execute(ResetColor).unwrap();
    
    let mut res = String::new();
    stdin().read_line(&mut res).expect("Did not enter a correct string");
    
    res.trim().to_string()
}
#[derive(Debug,PartialEq)]
pub enum PrinCommand{
    AICall,
    UnitTest,
    Issue
}

impl PrinCommand{
    pub fn print_agent_message(&self, agent_pos:&str , agent_statement:&str){
        let mut stdout = stdout();
        
        let statement_color = match self {
            PrinCommand::AICall => Color::Cyan,
            PrinCommand::UnitTest => Color::Magenta,
            PrinCommand::Issue => Color::Red,
        };
        stdout.execute(SetForegroundColor(Color::Green )).unwrap();
        print!("Agent {} ", agent_pos);
        
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);
        
        stdout.execute(ResetColor).unwrap();
    }
}


