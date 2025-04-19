use clap::{Parser, Subcommand};
use std::fs;
use std::io::prelude::*;
use std::env;
use std::path::PathBuf;

trait FileScaffolding{
    fn extension(&self) -> &'static str;
    fn template(&self) -> &'static str;
}

struct Python;
struct Cpp;
struct C;
struct Rust;
struct Java;
struct JavaScript;

#[derive(Parser)]
#[command(name = "scaffold", version, about)]
struct Information {
    #[command(subcommand)]
    cmd: Commands, // Inputted Command
    name: String, // The name the user wants the project to be
    //info_type: String, // d for directory or f for file
}

impl FileScaffolding for Python { 
    fn extension(&self) -> &'static str { ".py"}
    fn template(&self) -> &'static str {
        r#"if __name__ == "__main__":
            print("Hello, world!")
        "#
    }
}

impl FileScaffolding for Cpp { 
    fn extension(&self) -> &'static str { ".cpp"}
    fn template(&self) -> &'static str {
        r#" #include <iostream>    

            int main() {
                std::cout << "Hello, world!";
                return 0
            }
        "#
    }
}

impl FileScaffolding for C { 
    fn extension(&self) -> &'static str { ".c"}
    fn template(&self) -> &'static str {
        r#" #include <studio.h>  

            int main() {
                printf("Hello, world");
                return 0;
            } 
            
        "#
    }
}

impl FileScaffolding for Rust { 
    fn extension(&self) -> &'static str { ".rs"}
    fn template(&self) -> &'static str {
        r#"
            fn main(){
                println!("Hello, world!");
            }  
        "#
    }
}

impl FileScaffolding for Java { 
    fn extension(&self) -> &'static str { ".java"}
    fn template(&self) -> &'static str {
        r#"
            public class HelloWorld { 
                public static void main() {
                    System.out.println("Hello, world!");
                }  
            }
        "#
    }
}

impl FileScaffolding for JavaScript { 
    fn extension(&self) -> &'static str { ".js"}
    fn template(&self) -> &'static str {
        r#"
            alert("Hello, world!");
        "#
    }
}
#[derive(Subcommand)]
enum Commands {
    Cpy {name: String},
    Ccpp {name: String},
    Cc {name: String},
    Crs {name: String},
    Cjava {name: String},
    Cjs {name: String},
    Rem {name: String},
    Edit {name: String},
    Sdir {name: String},
    Rdir {name: String},
}

fn command_processing(command: &Commands, project_path: &PathBuf){

    match command{
        Commands::Cpy {name } => create_project(".py", &project_path).unwrap(),        // Creates a Python project
        Commands::Ccpp {name } => create_project(".cpp", &project_path).unwrap(),      // Creates a Cpp project
        Commands::Cc {name } => create_project(".c", &project_path).unwrap(),          // Creates a C project
        Commands::Crs {name } => create_project(".rs", &project_path).unwrap(),        // Creates a Rust project
        Commands::Cjava {name } => create_project(".java", &project_path).unwrap(),    // Creates a Java project
        Commands::Cjs {name } => create_project(".js", &project_path).unwrap(),        // Creates a JavaScript project
        Commands::Rem {name } => remove_file("not implemented"),                            // Removes a project or file from memory
        Commands::Edit {name } => edit_file("not implemented"),                             // Edits a file
        Commands::Sdir {name } => create_default_directory(&project_path),                        // Saves given directory as a defult directory for creating projects
        Commands::Rdir {name } => remove_default_directory(&project_path),                        // Removes the saved default directory and returns to current directory
        _ => println!("Command Type not recognized! Please use help for command types."), 
        // Error if there is no matching command
    };

}

fn create_project(file_type: &str, project_path: &PathBuf) -> std::io::Result<()>{
    fs::create_dir_all(project_path)?;

    let file_path = project_path.join(format!("{}{}", "main", file_type));
    let mut file = fs::File::create_new(&file_path)?;

    file.write(b"fn main() {
        println!(\"Hello World\")s
    }")?;

    println!("test");
    Ok(())
}

fn remove_file(name: &str) {
    println!("test");
}

fn edit_file(name: &str) {
    println!("test");
}

fn create_default_directory(directory: &PathBuf) {
    println!("test");
}

fn remove_default_directory(directory: &PathBuf) {
    println!("test");
}

fn main() -> std::io::Result<()> {
    let args = Information::parse();
    let dir_path = env::current_dir()?;
    let project_path = dir_path
                                .join(&args.name);

    command_processing(&args.cmd, &project_path);
    println!("Created {} in the directory {}!", &args.name, &dir_path.display());
    Ok(())
}
