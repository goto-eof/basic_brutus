pub mod core {
    use regex::Regex;

    use std::collections::HashMap;

    pub struct BasicBrutusError {
        pub code: i16,
        pub description: String,
    }

    impl BasicBrutusError {
        fn new(description: String, code: i16) -> Self {
            Self { description, code }
        }
    }

    fn global_trimer(command: &str) -> String {
        let re = Regex::new(r"\s+").unwrap();
        re.replace_all(command, " ").to_string()
    }

    pub fn parse(command: &str) -> Result<HashMap<String, String>, BasicBrutusError> {
        let trimmed = global_trimer(command);
        let splitted: Vec<&str> = trimmed.split(" ").collect();

        if splitted.len() % 2 != 0 {
            return Err(BasicBrutusError::new("Invalid command".to_string(), 1));
        }
        let mut commands_map: HashMap<String, String> = HashMap::new();
        for i in 0..splitted.len() {
            match splitted[i] {
                "-t" => commands_map.insert("uri".to_string(), splitted[i + 1].to_string()),
                "-d" => commands_map.insert("dictionary".to_string(), splitted[i + 1].to_string()),
                "-u" => commands_map.insert("username".to_string(), splitted[i + 1].to_string()),
                _ => None,
            };
        }
        is_command_well_formed(&commands_map)?;
        Ok(commands_map)
    }

    fn is_command_well_formed(
        commands_map: &HashMap<String, String>,
    ) -> Result<(), BasicBrutusError> {
        if commands_map.get("uri").is_none() || commands_map.get("dictionary").is_none() {
            return Err(BasicBrutusError {
                code: 3,
                description: "Invalid command".to_string(),
            });
        }
        Ok(())
    }
}
