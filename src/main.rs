mod pentry;

use crate::pentry::prompt;
use crate::pentry::read_passwords_from_file;
use crate::pentry::ServiceInfo;

fn clr() {
    println!("{}[23", 27 as char);
}


fn main(){
    clr();
    let ascii = r#"
    
                       __                                                     
_______ __ __  _______/  |_  _____________  ____   ________________    _____  
\_  __ \  |  \/  ___/\   __\ \____ \_  __ \/  _ \ / ___\_  __ \__  \  /     \ 
 |  | \/  |  /\___ \  |  |   |  |_> >  | \(  <_> ) /_/  >  | \// __ \|  Y Y  \
 |__|  |____//____  > |__|   |   __/|__|   \____/\___  /|__|  (____  /__|_|  /
                  \/         |__|               /_____/            \/      \/ 

    "#;

    println!("{ascii}");
    loop{
        println!("password manager menu");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search Entry");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim(){
            "1" => {

                clr();
                let entry = ServiceInfo::new(
                    prompt("Service: :"),
                    prompt("username :"),
                    prompt("Password :")
                );
                println!("Entry added successfully");
                entry.write_to_file();
            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                eprintln!("Error reading passwords:{}",err); 
                Vec::new()
                });
                for item in &services{
                    println!(
                        "Service = {}
                        - username : {}
                        - password : {}",
                        item.service, item.username, item.password
                    )
                } 
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords:{}",err); 
                    Vec::new()
                    });
                let search = prompt("Search :");
                for item in &services{
                    if item.service.as_str() == search.as_str(){
                        println!(
                            "Service = {}
                            - username : {}
                            - password: {}",
                            item.service, item.username, item.password
                        );    
                    }
                }
            }
            "4" => {
                clr();
                println!("Goodbye!");
                break;

            }
            _ => println!("Invalid choice.")
        }
        println!("\n\n");
    }
}