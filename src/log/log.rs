use colored::*;
use std::{thread, time::Duration};

use std::io::Write;
use std::io;

use crate::core::project::Project;
use crate::core::project::view_mission;

#[derive(Debug)]
pub struct Log {}

pub trait LogF {
    fn new() -> Self;

    fn hey(&self, msg: &str);
    fn hey_mw(&self, msg: &str);
    fn ascii_myway(&self);
    fn hey_project(&self, project: &Project, complex: bool);

    fn _quest(&self, question: &str) -> String;
    fn quest_mandatory(&self, question: &str, default: &str) -> String;
    fn quest_option(&self, question: &str, options: Vec<&str>, default: &str) -> String;
}


impl LogF for Log {
    fn new() -> Self { Self {} }

    fn hey(&self, msg: &str) {
        println!("{}", msg);
    }

    fn hey_mw(&self, msg: &str) {
        println!("{} {}", "[MW]".bright_purple(), msg.italic());
    }

    fn hey_project(&self, project: &Project, complex: bool) {
        println!("\t{}", project.description.dimmed().italic());
        println!("\tMission: \"{}\"", view_mission(&project.mission).italic().yellow());
        if complex {
            println!("\tCreated at: {}", project.time_created.italic());
            println!("\tStack(s):");
            if project.stack.len() != 0 {
                for i in project.stack.iter() {
                    println!("\t- {} ", i.bold())
                }
            } else {
                println!("\t  {}", "No Stack(s)".bold())
            }
        }
    }


    fn _quest(&self, question: &str) -> String {
        print!("{}", question.bold());

        io::stdout().flush().expect("Failed to load stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error in read");

        input.trim().to_string()
    }

    fn quest_mandatory(&self, question: &str, default: &str) -> String {
        print!("{} ({}{}): ", question.bold(), "default:".dimmed(), format!("{}", default).dimmed());

        io::stdout().flush().expect("Failed to load stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error in read");
        
        let trimmed = input.trim();

        if trimmed.is_empty() {
            default.to_string()
        } else {
            trimmed.to_string()
        }
    }

    fn quest_option(&self, question: &str, options: Vec<&str>, default: &str) -> String {
        print!("{} ({}{}): ", question.bold(), "default:".dimmed(), format!("{}", default).dimmed());

        io::stdout().flush().expect("Failed to load stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error in read");

        let trimmed = input.trim();

        if trimmed.is_empty() {
            default.to_string()
        } else {
            if options.contains(&trimmed) {
                trimmed.to_string()
            } else {
                default.to_string()
            }
        }
    }

    fn ascii_myway(&self) {
        let credits_ascii = "Credits: ASCII Generated with 'Text to ASCII Art Generator by PATORJK'";
        let logo = r#"
 _____ ______       ___    ___ ___       __   ________      ___    ___ 
|\   _ \  _   \    |\  \  /  /|\  \     |\  \|\   __  \    |\  \  /  /|
\ \  \\\__\ \  \   \ \  \/  / | \  \    \ \  \ \  \|\  \   \ \  \/  / /
 \ \  \\|__| \  \   \ \    / / \ \  \  __\ \  \ \   __  \   \ \    / / 
  \ \  \    \ \  \   \/  /  /   \ \  \|\__\_\  \ \  \ \  \   \/   / /  
   \ \__\    \ \__\__/  / /      \ \____________\ \__\ \__\__/   / /    
    \|__|     \|__|\___/ /        \|____________|\|__|\|__|\____/ /     
                  \|___|/                                 \|____|/      
                                                                       
        "#;

        let start_color = (255, 0, 255);
        let end_color = (0, 255, 255); 

        let lines: Vec<&str> = logo.lines().collect();
        let num_lines = lines.len();

        for (i, line) in lines.iter().enumerate() {

            let r = start_color.0 + (end_color.0 - start_color.0) * i as i32 / num_lines as i32;
            let g = start_color.1 + (end_color.1 - start_color.1) * i as i32 / num_lines as i32;
            let b = start_color.2 + (end_color.2 - start_color.2) * i as i32 / num_lines as i32;

            // Imprime a linha com a cor calculada
            println!("{}", line.truecolor(r as u8, g as u8, b as u8));
            thread::sleep(Duration::from_millis(100));
        }
        
        println!();
        println!("{}", credits_ascii.italic());
        println!();
        println!("{}: {}", "Developed by".italic(), "Gabriel \"dotxav\" Xavier".bold());
        println!("{}: {}", "Security & Encryption by ".italic(), "Tequel v0.4.5".bold());
    }
}