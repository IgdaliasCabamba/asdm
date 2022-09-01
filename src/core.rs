use crate::modules::{console};
use std::process::Command;

pub fn new_process(bin: &str, args: &Vec<std::string::String>) -> std::process::ExitStatus {
    let mut console_api = console::Console::new();

    console_api.print_start();

    let process = Command::new(bin)
        .args(args)
        .output()
        .expect(&console_api.print_exeption(bin.to_owned()));

    let out = std::string::String::from_utf8(process.stdout);
    console_api.print_output(out.unwrap());

    if process.status.success() {
        let status_code = process.status.code().unwrap().to_string();
        let mut bin_args = String::new();

        for i in args {
            let a: &str = i.as_ref();
            bin_args.push_str(a);
            bin_args.push_str("\n\t");
        }
        console_api.print_exit_status_sucess(bin.to_owned(), status_code, bin_args)
    } else {
        console_api.print_exit_status_fail(bin.to_owned())
    }

    console_api.print_end();

    return process.status;
}