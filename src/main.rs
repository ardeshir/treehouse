use std::io::stdin;

#[derive(Debug)]
enum FriendAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

fn get_name() -> String {
    let mut name = String::new();

    stdin().read_line(&mut name).expect("Failed to get name");
    name.trim().to_lowercase()
}

fn main() {
    #[derive(Debug)]
    struct Friends {
        name: String,
        greeting: String,
        action: FriendAction,
        age: i8,
    }
    impl Friends {
        fn new(name: &str, greeting: &str, action: FriendAction, age: i8) -> Self {
            Self {
                name: name.to_lowercase(),
                greeting: greeting.to_string(),
                action,
                age,
            }
        }
        fn greet_visitor(&self) {
            match &self.action {
                FriendAction::Accept => println!("Hello {},{}", self.name, self.greeting),
                FriendAction::AcceptWithNote { note } => {
                    println!("Hi {}, {}", self.name, self.greeting);
                    println!("{}", note);
                }
                FriendAction::Probation => println!("What's up {},{}", self.name, self.greeting),
                FriendAction::Refuse => println!("Sorry, {},{}", self.name, self.greeting),
            }
            if self.age < 21 {
                println!("Not old enought to drink here.");
            }
        }
    }

    let mut friends_names = vec![
        Friends::new("ardeshir", " welcome back!", FriendAction::Accept, 51),
        Friends::new("roxy", " Woof woof back!", FriendAction::Accept, 21),
        Friends::new(
            "anoush",
            " nice to see you again!",
            FriendAction::Probation,
            10,
        ),
        Friends::new(
            "kayhan",
            " great to have to you back!",
            FriendAction::AcceptWithNote {
                note: String::from("Vegitarian and needs yogurt and chips!"),
            },
            12,
        ),
        Friends::new(
            "joe",
            " What are you doing here, please leave now!",
            FriendAction::Refuse,
            48,
        ),
    ];

    // Start the welcome loop ask for names and greet
    loop {
        println!("Welcome, what's your name? (Leave empty, press Enter to leave)");
        // grab the name from stdio and store in name
        let name = get_name();

        let friends = friends_names.iter().find(|friend| friend.name == name);

        match friends {
            Some(friend) => friend.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                // Ends the welcome loop and println all the names.
                } else {
                    println!("You are not on the VIP list. Please subscribe to our clue to enter!");
                    friends_names.push(Friends::new(
                        &name,
                        "New Friend added!",
                        FriendAction::Probation,
                        0,
                    ));
                }
            }
        }
    }
    // end of loop, break checking people into TreeHouse and prints names
    println!("The final list of friends:");
    println!("{:#?}", friends_names);
} // end of main
