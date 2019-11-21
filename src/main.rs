mod exercises {
  pub mod hashmaps {
    pub mod program {
      use std::collections::HashMap;
      use std::io;

      fn print_with_indent(text: String, level: usize) {
        println!("{}{}", " ".repeat(level), text);
      }

      struct NaiveStore {
        state: HashMap<String, Vec<String>>,
      }

      impl NaiveStore {
        pub fn new() -> Self {
          NaiveStore {
            state: HashMap::new(),
          }
        }

        pub fn add(&mut self, employee: String, department: String) {
          let values = self.state.entry(department).or_insert(vec![]);
          values.push(employee);
          values.sort();
        }

        pub fn print_by_department(&self, department: String) {
          if self.state.len() == 0 {
            print_with_indent(
              "Nothing to show yet, make sure to add a new entry with :add".to_owned(),
              2,
            );
            return;
          }
          match self.state.get(&department) {
            Some(employees) => {
              employees
                .iter()
                .for_each(|employee| print_with_indent(format!("* {}", employee), 2));
            }
            None => {
              print_with_indent("Incorrect department name:".to_owned(), 2);
              self
                .state
                .keys()
                .for_each(|department| print_with_indent(format!("* {}", department), 4))
            }
          }
        }

        pub fn print_all(&self) {
          if self.state.len() == 0 {
            print_with_indent(
              "Nothing to show yet, make sure to add a new entry with :add".to_owned(),
              2,
            );
            return;
          }
          self.state.iter().for_each(|(department, employees)| {
            print_with_indent(format!("# {}", department), 2);
            employees.iter().for_each(|employee| {
              print_with_indent(format!("* {}", employee), 4);
            });
          })
        }
      }

      #[derive(Debug)]
      pub enum Command {
        Add(String, String),
        List(String),
        All,
        Quit,
        Help,
      }

      fn parse_input(input: String) -> Option<Command> {
        let parsed_input = input.trim();
        let mut cmd_parts = parsed_input.split_ascii_whitespace();

        match cmd_parts.next()? {
          ":add" => {
            let person = cmd_parts.next()?;
            let department = cmd_parts.next()?;
            Some(Command::Add(String::from(person), String::from(department)))
          }
          ":list" => {
            let department = cmd_parts.next()?;
            Some(Command::List(String::from(department)))
          }
          ":all" => Some(Command::All),
          ":quit" => Some(Command::Quit),
          ":q" => Some(Command::Quit),
          ":help" => Some(Command::Help),
          ":h" => Some(Command::Help),
          _ => None,
        }
      }

      fn handle_command(store: &mut NaiveStore, command: Command) {
        match command {
          Command::Add(employee, department) => {
            store.add(employee, department);
            println!("{}", true);
          }
          Command::List(department) => {
            store.print_by_department(department);
          }
          Command::All => {
            store.print_all();
          }
          _ => {
            println!("Could not handle the given command! (:help)");
          }
        }
      }

      fn help() -> String {
        let commands = [
          ":add <Person> <Department>",
          ":list <Department>",
          ":all",
          ":quit (:q)",
          ":help (:h)",
        ];

        commands
          .iter()
          .map(|command_str| format!("* {}", command_str))
          .fold(String::from(""), |acc, command_str| {
            format!("{}{}\n", acc, command_str)
          })
      }

      pub fn cli() {
        let mut store: NaiveStore = NaiveStore::new();

        println!(
          "Welcome to the system. You can perform the following operations:\n{}",
          help()
        );

        loop {
          let mut input = String::new();

          io::stdin().read_line(&mut input).unwrap();

          match parse_input(input) {
            Some(Command::Help) => println!("{}", help()),
            Some(Command::Quit) => break,
            Some(command) => handle_command(&mut store, command),
            None => {
              print_with_indent("Incorrect given input (:help)".to_owned(), 2);
            }
          }
        }

        println!("The session is being terminated...");
      }
    }

    pub fn main() {
      program::cli();
    }
  }

  pub mod strings {
    fn convert_to_pig_latin(word: &str) -> String {
      let first_char = word.chars().next().unwrap();
      if first_char == 'a'
        || first_char == 'A'
        || first_char == 'e'
        || first_char == 'E'
        || first_char == 'i'
        || first_char == 'I'
        || first_char == 'o'
        || first_char == 'O'
        || first_char == 'u'
        || first_char == 'U'
      {
        format!("{}-hay", &word[0..])
      } else {
        format!("{}-{}ay", &word[1..], first_char)
      }
    }

    pub fn main() {
      assert_eq!("ola-Hay", convert_to_pig_latin("Hola"));
      assert_eq!("irst-Fay", convert_to_pig_latin("First"));
      assert_eq!("Apple-hay", convert_to_pig_latin("Apple"));
    }
  }

  pub mod vectors {
    fn average(vector: Vec<i32>) -> i32 {
      let len = vector.len() as i32;
      let sum = vector.into_iter().fold(0, |acc, value| acc + value);
      sum / len
    }

    fn median(vector: Vec<i32>) -> i32 {
      let len = vector.len();
      let is_even = len % 2 == 0;
      let middle = ((len + 1) / 2) - 1 as usize;
      let mut cloned_vec = vector.clone();

      cloned_vec.sort();

      if is_even {
        average(vec![*&cloned_vec[middle], *&cloned_vec[middle + 1]])
      } else {
        *&cloned_vec[middle]
      }
    }

    fn mode(vector: Vec<i32>) -> i32 {
      use std::collections::HashMap;
      let mut collector: HashMap<i32, i32> = HashMap::new();

      vector
        .into_iter()
        .fold((0, 0), |(k, max), value| {
          let count = collector.entry(value).or_insert(0);
          *count = *count + 1;
          if *count > max {
            return (value, *count);
          }
          (k, max)
        })
        .0
    }

    pub fn main() {
      // Average
      assert_eq!(5, average(vec![5, 4, 12, 2, 1, 6]));
      assert_eq!(10, average(vec![5, 10, 20, 5, 10]));
      assert_eq!(10, average(vec![4, 7, 10, 13, 16]));
      // Median
      assert_eq!(2, median(vec![1, 4, 2, 5, 0]));
      assert_eq!(30, median(vec![10, 20, 40, 50]));
      // Mode
      assert_eq!(2, mode(vec![1, 2, 2, 5, 0]));
      assert_eq!(30, mode(vec![2, 20, 30, 30]));
    }
  }
}

fn main() {
  exercises::vectors::main();
  exercises::strings::main();
  exercises::hashmaps::main();
}
