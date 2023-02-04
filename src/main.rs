mod checker;
mod interpreter;
mod runner;
mod utils;

use utils::write_template;
use reqwest;


fn main() -> () {
    let command = std::env::args().nth(1);
    let argument = std::env::args().nth(2);

    match command {
        Some(command)=>{
            match command.as_str() {
                "help"=>{
                    println!("Show certus help message")
                },
                "generate"=>{
                    println!("Generate certus template file");
                    match argument {
                        Some(argument)=>{
                            match argument.as_str() {
                                _=>{
                                    let template_path = std::path::Path::new(&argument);
                                    if template_path.exists(){
                                        println!("A file with the same name already exists");
                                        return
                                    }
                                    let mut template_file = std::fs::File::create(template_path).expect("Something went wrong 🥲");
                                    write_template(&mut template_file);
                                }
                            }
                        },
                        None=>{
                            // let template_path = std::path::Path::new("template.certus");
                            println!("You must provide a name. Run `certus help` to see certus documentation")
                        }
                    }
                },
                _=>{
                    //User wants to run a file or a directory
                    println!("{}", command)
                }
            }
        },
        None=>{
            println!("Show certus help message");
        }
    }
}
