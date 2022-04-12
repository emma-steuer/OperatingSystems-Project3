
use std::process::Command;
use std::path::Path;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::env;

fn main(){
    let mut path_list: Vec<String> = vec!["/bin".to_string()]; // = []

    loop {
        print!("ccsh> ");
        let mut all_commands = Vec::new();
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" & ").peekable();

        while let Some(command) = commands.next()  {

            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek()
                        .map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                },
                "path" => {
                    path_list = args.map(ToString::to_string).collect();
                    eprintln!("{:?}", path_list);
                }
                "exit" => return,
                command => {

                    for direc in &path_list {
                        if Path::new(direc).join(command).exists() {
                    
                            let output = Command::new(Path::new(direc).join(command))
                                .args(args)
                                .spawn();
                            match output {
                                Ok(output) => { all_commands.push(output); },
                                Err(e) => {
                                    eprintln!("{}", e);
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

    }
}