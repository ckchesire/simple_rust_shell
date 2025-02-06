// Create a default reedline object to handle user input

use reedline::{DefaultPrompt, Reedline, Signal};

fn welcome_message() {
    println!(
        "{}",
        r#"
        ____       _          _ _
        |  _ \ _ __(_)_  _____| | |
        | |_) | '__| \ \/ / _ \ | |
        |  __/| |  | |>  <  __/ | |
        |_|   |_|  |_/_/\_\___|_|_|
        Welcome to RustShell!
        "#
    )
}

fn main() {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    welcome_message();
    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                println!("[rshell]$ {}", buffer);
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
        }
    }
}
