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
    // Start the program greet and ask for name
    println!("Hello, What's your name?");

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


    struct Friends {
        name: String,
        greeting: String,
    }

    let friends_names = [
        Friends::new("ardeshir", "Hello Ardeshir!"),
        Friends::new("anoush", "Hello Anoush!"),
        Friends::new("kayhan", "Hello Kayhan!"),
    ];

    let name = get_name();
    
    let friends = friends_names
       .iter()
       .find(|friend| friend.name == name);
    
    match friends {
         Some(friend) => friend.greet_visitor(),
         None => println!("You are not on the VIP list. Please subscribe to clue!")
    }

    /* let mut allowed = false;

    for friend in &friends_names {
        if friend == &name {
        allowed = true;
        }
    }

    if allowed {
        println!("Welcome back {}!", name);
    } else {
        println!("Sorry {}, you're not allowed!", name);
    } 
    */
}
