use clap::{Parser, Subcommand};
use std::{
    fs, 
    io::{self, Write},
    path::{Path, PathBuf}
};

trait FileScaffolding{
    //Using the trait functionality to make a template for each language, I would like to make a project in
    fn extension(&self) -> &'static str; // The file extension for the language
    fn template(&self) -> &'static str; // The templated "Hello, world!" implemenation for each language
}

// Supported Languages
struct Python;
struct Cpp;
struct C;
struct Rust;
struct Java;
struct JavaScript;
//////////////////////

#[derive(Parser)]
#[command(name = "scaffold", version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands, // Inputted Command
    //info_type: String, // d for directory or f for file
}

//Implements the scaffolding trait for Python
impl FileScaffolding for Python { 
    fn extension(&self) -> &'static str { ".py"}
    fn template(&self) -> &'static str {
        r##"if __name__ == "__main__":
    print("Hello, world!")
        "##
    }
}

//Implements the scaffolding trait for C++
impl FileScaffolding for Cpp { 
    fn extension(&self) -> &'static str { ".cpp"}
    fn template(&self) -> &'static str {
        r##"#include <iostream>   
         
int main() {
    std::cout << "Hello, world!";
    return 0;
}
        "##
    }
}

//Implements the scaffolding trait for C
impl FileScaffolding for C { 
    fn extension(&self) -> &'static str { ".c"}
    fn template(&self) -> &'static str {
        r##"#include <stdio.h>  

int main() {
    printf("Hello, world");
    return 0;
}            
         "##
    }
}

//Implements the scaffolding trait for Rust
impl FileScaffolding for Rust { 
    fn extension(&self) -> &'static str { ".rs"}
    fn template(&self) -> &'static str {
        r##"
fn main(){
    println!("Hello, world!");
    }  
        "##
    }
}

//Implements the scaffolding trait for Java
impl FileScaffolding for Java { 
    fn extension(&self) -> &'static str { ".java"}
    fn template(&self) -> &'static str {
        r##"
public class HelloWorld { 
    public static void main() {
        System.out.println("Hello, world!");
    }  
}
        "##
    }
}

//Implements the scaffolding trait for JavaScript
impl FileScaffolding for JavaScript { 
    fn extension(&self) -> &'static str { ".js"}
    fn template(&self) -> &'static str {
        r##"
alert("Hello, world!");
        "##
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
    Rem {file: PathBuf},
    Edit {file: PathBuf},
    Sdir {dir: PathBuf},
    Rdir {dir: PathBuf},
}

fn command_processing(cmd: &Commands) -> io::Result<()>{

    match cmd{
        // Creates a Python project
        Commands::Cpy {name } => {let path = Path::new(name);
                                            create_project(Python {}, path)?;
                                            println!("Created Python Project '{}' in '{}'", name, path.display());},  

        // Creates a C++ project
        Commands::Ccpp {name } => {let path = Path::new(name);
                                            create_project(Cpp {}, path)?;
                                            println!("Created Python Project '{}' in '{}'", name, path.display());},

        // Creates a C project
        Commands::Cc {name } => {let path = Path::new(name);
                                            create_project(C {}, path)?;
                                            println!("Created Python Project '{}' in '{}'", name, path.display());},

        // Creates a Rust project
        Commands::Crs {name } => {let path = Path::new(name);
                                            create_project(Rust {}, path)?;
                                            println!("Created Python Project '{}' in '{}'", name, path.display());},    

        // Creates a Java project
        Commands::Cjava {name } => {let path = Path::new(name);
                                            create_project(Java {}, path)?;
                                            println!("Created Python Project '{}' in '{}'", name, path.display());}, 

        // Creates a JavaScript project
        Commands::Cjs {name } => {let path = Path::new(name);
                                            create_project(JavaScript {}, path)?;
                                            println!("Created Python Project '{}' in '{}'", name, path.display());},  

        // Removes a project or file from memory
        Commands::Rem {file } => remove_file("not implemented"),                            
        Commands::Edit {file } => edit_file("not implemented"),                // Edits a file
        Commands::Sdir {dir } => create_default_directory(),                        // Saves given directory as a defult directory for creating projects
        Commands::Rdir {dir } => remove_default_directory(),                        // Removes the saved default directory and returns to current directory
        // Error if there is no matching command
    };

    Ok(())
}

fn create_project<S: FileScaffolding, P: AsRef<Path>>(sc: S, project_path: P) -> io::Result<()> {
    let project_path = project_path.as_ref();
    fs::create_dir_all(project_path)?;

    let file_path = project_path.join(format!("main{}", sc.extension()));
    let mut file = fs::File::create_new(&file_path)?;

    file.write_all(sc.template().as_bytes())?;

    Ok(())
}

fn remove_file(name: &str) {
    println!("test");
}

fn edit_file(name: &str) {
    println!("test");
}

fn create_default_directory() {
    println!("test");
}

fn remove_default_directory() {
    println!("test");
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    command_processing(&cli.cmd)?;
    Ok(())
}
