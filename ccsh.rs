
use std::process::Command;
use std::path::Path;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::env;
use  std::fs;


#[derive(PartialEq)]
enum ShouldExitOrNot {
    Exit,
    KeepGoing,
}

fn run_one_line_commands(line_of_commands: &str, path_list: &mut Vec<String>) -> ShouldExitOrNot {

    let mut commands = line_of_commands.trim().split(" & ").peekable();
    let mut all_commands = Vec::new();

    while let Some(command) = commands.next()  {

        let mut parts = command.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek()
                    .map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(_e) = env::set_current_dir(&root) {
                    eprintln!("An error has occured");
                }

            },
            "path" => {
                *path_list = args.map(ToString::to_string).collect();
                
            }
            "exit" => return ShouldExitOrNot::Exit,
            command => {

                for direc in &*path_list {
                    if Path::new(direc).join(command).exists() {
                
                    
                        let output = Command::new(Path::new(direc).join(command))
                            .args(args)
                            .spawn();
                        match output {
                            Ok(output) => { all_commands.push(output); },
                            Err(_e) => {
                                eprintln!("An error has occurred"); 
                            },
                        };
                        break;
                    }
                }
            }
            
        }


        
    }

    for mut cmd in all_commands {
        let _ = cmd.wait();

    }

    ShouldExitOrNot::KeepGoing
}


fn main(){
    let mut path_list: Vec<String> = vec!["/bin".to_string()]; 

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {

        let contents = fs::read_to_string(&args[1])
            .expect("An error has occured");

        for line in contents.split("\n"){
            if run_one_line_commands(line, &mut path_list) == ShouldExitOrNot::Exit {
                return;
            } 
            
        }
        
        
    }

    else {
        loop {
            print!("ccsh> ");
            
            stdout().flush();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            if run_one_line_commands(&input, &mut path_list) == ShouldExitOrNot::Exit {
                return;
            }

            
        }
    }
}