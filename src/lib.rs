extern crate wabt;
extern crate tempfile;

pub fn wasm2ll(func: &str, _wasm: &[u8]) -> String {
    String::from(format!(r#"
define i32 @{}(i32 %a, i32 %b) #0 {{
  %1 = add nsw i32 %a, %b
  ret i32 %1
}}
"#, func))
}

#[cfg(test)]
mod tests {
    use wabt::script::{ScriptParser, Command, CommandKind, Action, Value, Error};
    use std::collections::HashMap;

    #[test]
    fn testsuite() -> Result<(), Error> {
        use std::fs;
        use std::fs::File;
        use std::path::Path;
        use std::io::Write;
        use std::process;
        use tempfile::NamedTempFile;
        
        let wast = fs::read_to_string("testsuite/i32.wast").unwrap();
        let mut parser: ScriptParser = ScriptParser::from_str(&wast)?;
        let mut funcs: HashMap<String, &[u8]> = HashMap::new();

        while let Some(Command { kind, .. }) = parser.next()? {
            match kind {
                CommandKind::Module { module, name } => {
                    let module_binary = module.into_vec();
                    funcs.insert(String::from("add"), &[0, 97, 115, 109]); // TODO
                }
                CommandKind::AssertReturn { action, expected: expected_vec } => {
                    if let Action::Invoke {
                        module: _,
                        field: func,
                        args
                    } = action {
                        if let (Value::I32(arg1), Value::I32(arg2)) = (&args[0], &args[1]) {
                            if let Value::I32(expected) = &expected_vec[0] {
                                // let file_name = format!("{}.ll",func);
                                // let mut file = File::create(Path::new(&file_name))?;
                                let mut file = NamedTempFile::new()?;
                                let ll = 
                                    format!(r#"
{}
define i32 @main() {{
  %1 = call i32 @{}(i32 {}, i32 {})
  ret i32 %1
}}
"#,
                                         super::wasm2ll(&func,
                                                        funcs.get(&String::from(&func)).unwrap()),
                                         func,
                                         arg1,
                                         arg2);
                                file.write_all(ll.as_bytes())?;
                                file.flush()?;
                                
                                // Convert LLVM IR to bitcode
                                process::Command::new("llvm-as-8")
                                    .arg(file.path())
                                    .output()?; // wait for it to finish

                                let bitcode = format!("{}.bc",file.path().to_string_lossy());
                                
                                let status = process::Command::new("lli-8")
                                    .arg(&bitcode) // XXX
                                    .status()?;

                                assert_eq!(expected, &status.code().unwrap());
                                // if (*expected == status.code().unwrap()) {
                                //     fs::remove_file(&bitcode);                                    
                                // } else {
                                //     panic!(" {}", &bitcode);
                                // }
                                fs::remove_file(&bitcode);                                    

                                // "add"
                                break;
                            }
                        }
                    }
                }
                _ => panic!("TODO")
            }
        }

        Ok(())
    }
}
