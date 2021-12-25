use std::io::stdin;

fn get_name() -> String {
    let mut name = String::new();

    stdin()
        .read_line(&mut name)
        .expect("Failed to get name");
    name
      .trim()
      .to_lowercase()
}

fn main() {

    impl Friends {
        fn new ( name: &str, greeting: &str) -> Self {
            Self {
                name: name.to_lowercase(),
                greeting: greeting.to_string(),
            }
        }
        fn greet_visitor (&self) {
            println!("{}", self.greeting);
        }
    }

    #[derive(Debug)]
    struct Friends {
        name: String,
        greeting: String,
    }

    let mut friends_names = vec![
        Friends::new("ardeshir", "Hello Ardeshir!"),
        Friends::new("anoush", "Hello Anoush!"),
        Friends::new("kayhan", "Hello Kayhan!"),
    ];


    // Start the program greet and ask for name
    loop {
    println!("Hello, What's your name? (Leave empty and press Enter to leave)");
    let name = get_name();
    let friends = friends_names
       .iter()
       .find(|friend| friend.name == name);
    
        match friends {
          Some(friend) => friend.greet_visitor(),
          None => {
           if name.is_empty() {
               break;
            } else {
             println!("You are not on the VIP list. Please subscribe to clue!");
             friends_names.push(Friends::new(&name, "New Friend added!"));
            }
         }
       } 
   }
   println!("The final list of friends:");
   println!("{:#?}", friends_names);
} // end of main 
