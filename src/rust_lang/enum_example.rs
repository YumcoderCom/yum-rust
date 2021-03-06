mod test {
    #[test]
    fn test_enum_simple_usage() {
        #[derive(Debug)]
        enum Event {
            Update,
            Delete,
            Unknown,
        }
        type Message = String;
        fn parse_log(line: &str) -> (Event, Message) {
            // Vec<_> asks Rust to infer the elements’ type.
            let parts: Vec<_> = line.splitn(2, ' ').collect(); // 2:  returning at most `n` items.
            if parts.len() == 1 {
                return (Event::Unknown, String::from(line));
            }
            let event = parts[0];
            let rest = String::from(parts[1]);

            match event {
                "UPDATE" | "update" => (Event::Update, rest),
                "DELETE" | "delete" => (Event::Delete, rest),
                _ => (Event::Unknown, String::from(line)),
            }
        }

        let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

        for line in log.lines() {
            let parse_result = parse_log(line);
            println!("{:?}", parse_result);
        }
    }
}
