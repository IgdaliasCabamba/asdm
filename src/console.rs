use termimad::{*, minimad::TextTemplate};
use chrono::{Timelike, Utc};
use crossterm;

pub(crate) struct Console {
    console: MadSkin,
}
impl Console {
    pub fn new() -> Console {
        Console {
            console: MadSkin::default(),
        }
    }

    pub fn print_start(&self) {
        let now = Utc::now();
        let hour = now.hour();

        println!(
            "Starting at {:02}:{:02}:{:02}",
            hour,
            now.minute(),
            now.second(),
        );
    }

    pub fn print_output(&mut self, stdout: String) {
        let output_template = TextTemplate::from(r#"* OUTPUT: **${text}**"#);
        let mut output_expander = output_template.expander();

        output_expander.set("text", stdout.as_str());
        self.console.print_expander(output_expander);
    }

    pub fn print_exit_status_sucess(&mut self, bin: String, status_code: String, bin_args: String) {
        self.console.bold.set_fg(crossterm::style::Color::Yellow);
        self.console.italic.set_fg(crossterm::style::Color::Blue);
        self.console
            .paragraph
            .set_fg(crossterm::style::Color::Green);

        let text_template = TextTemplate::from(
            r#"
        * ran: **${program}**
        * code: **${status-code}**
        * *args*: **${program-args} **
        "#,
        );
        let mut expander = text_template.expander();
        expander
            .set("program", &bin)
            .set("status-code", &status_code)
            .set("program-args", &bin_args);
        self.console.print_expander(expander);
    }
    pub fn print_exit_status_fail(&mut self, bin: String) {
        self.console.bold.set_fg(crossterm::style::Color::Red);
        self.console.italic.set_fg(crossterm::style::Color::Grey);

        let text_template = TextTemplate::from(r#"**Failed to execute**: *${program}*"#);
        let mut expander = text_template.expander();

        expander.set("program", &bin);
        self.console.print_expander(expander);
    }

    pub fn print_end(&self) {
        let now = Utc::now();
        let hour = now.hour();

        println!(
            "Finishing at {:02}:{:02}:{:02}",
            hour,
            now.minute(),
            now.second(),
        );
    }
    pub fn print_exeption(&mut self, bin: String) -> String {
        let base_excpetion_msg: String = "Failed to load ".to_owned();
        let excpetion_log = base_excpetion_msg + &bin;
        return excpetion_log;
    }
    pub fn log(&self, msg:String){
        let text_template = TextTemplate::from(r#"**${message}**"#);
        let mut expander = text_template.expander();

        expander.set("message", &msg);
        self.console.print_expander(expander);
    }
    pub fn print_error(&mut self, msg:String){
        self.console.bold.set_fg(crossterm::style::Color::Red);       
        self.log(msg)
    }
    pub fn print_warning(&mut self, msg:String){
        self.console.bold.set_fg(crossterm::style::Color::Yellow);
        self.log(msg)
    }
}
