#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use derive_builder::Builder;
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    #[builder(each = "env")]
    env: Vec<String>,
    current_dir: Option<String>,
}
pub struct CommandBuilder {
    executable: std::option::Option<String>,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}
impl CommandBuilder {
    pub fn executable(&mut self, executable: String) -> &mut Self {
        self.executable = Some(executable);
        self
    }
    pub fn args(&mut self, args: Vec<String>) -> &mut Self {
        self.args = Some(args);
        self
    }
    pub fn arg(&mut self, arg: String) -> &mut Self {
        self.args.push(arg);
        self
    }
    pub fn env(&mut self, env: String) -> &mut Self {
        self.env.push(env);
        self
    }
    pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
        self.current_dir = Some(current_dir);
        self
    }
    pub fn build(&mut self) -> Result<Command, Box<dyn std::error::Error>> {
        Ok(Command {
            executable: self.executable.clone().ok_or("executable is not set")?,
            args: self.args.clone(),
            env: self.env.clone(),
            current_dir: self.current_dir.clone(),
        })
    }
}
impl Command {
    fn builder() -> CommandBuilder {
        CommandBuilder {
            executable: None,
            args: ::alloc::vec::Vec::new(),
            env: ::alloc::vec::Vec::new(),
            current_dir: None,
        }
    }
}
fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .arg("build".to_owned())
        .arg("--release".to_owned())
        .build()
        .unwrap();
    {
        match (&command.executable, &"cargo") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &[
                            "assertion failed: `(left == right)`\n  left: `",
                            "`,\n right: `",
                            "`",
                        ],
                        &match (&&*left_val, &&*right_val) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
            }
        }
    };
    {
        match (&command.args, &<[_]>::into_vec(box ["build", "--release"])) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &[
                            "assertion failed: `(left == right)`\n  left: `",
                            "`,\n right: `",
                            "`",
                        ],
                        &match (&&*left_val, &&*right_val) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
            }
        }
    };
}
