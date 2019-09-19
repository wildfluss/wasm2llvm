extern crate wabt;

#[cfg(test)]
mod tests {
    use wabt::script::{ScriptParser, Command, CommandKind, Action, Value, Error};

    #[test]
    fn it_works() -> Result<(), Error> {
        use std::fs;
        
        let wast = fs::read_to_string("testsuite/i32.wast").unwrap();
        let mut parser: ScriptParser = ScriptParser::from_str(&wast)?;
        while let Some(Command { kind, .. }) = parser.next()? {
            if let CommandKind::AssertReturn { action, expected } = kind {
                if let Action::Invoke {
                    module,
                    field,
                    args
                } = action {
                    println!("{} {:?} {:?}", field, args, expected);

                    break;
                }
            }
        }

        Ok(())
    }
}
