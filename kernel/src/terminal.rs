use alloc::string::{String, ToString};
use spin::Mutex;
use crate::serial::emergency_message;

pub struct CommandBuffer{
    command : [char; 128],
    len : u8
}

impl CommandBuffer {
    pub const fn new() -> CommandBuffer {
        let command :[char; 128] = ['\0'; 128];
        let len = 0;
        CommandBuffer{command, len}
    }

    pub fn push(&mut self, c: char) {
        if self.len < 128{
            self.command[self.len as usize] = c;
            self.len+=1;
        }
        else{
            emergency_message("exceeded capacity");
        }
    }

    pub fn delete(&mut self){
        if(self.len > 0){self.len-=1;}
        else{emergency_message("nothing to delete")};
    }

    pub fn current_command(&mut self) -> String {
        let text: String = self.command.iter().copied().take(self.len as usize).collect();
        return text;
    }

    pub fn flush(&mut self){
        self.command = ['\0'; 128];
        self.len = 0;
    }
}

pub struct CommandParser{
    pub command_buffer: CommandBuffer,
}

impl CommandParser{
    pub const fn new() -> CommandParser {
        let command_buffer = CommandBuffer::new();
        return CommandParser{command_buffer};
    }

    fn match_command_part(string :String) -> bool{
        if(string == "h" || string == "he" || string == "hel" || string == "s"|| string == "sa"){
            return true;
        }
        else{
            return false;
        }
    }
    fn match_command(string: String) -> bool{
        if(string == "help" || string == "say" || string == "?") {
            return true;
        }
        return false;
    }

    pub fn lex(&mut self){
        let str = self.command_buffer.current_command();
        let temp  = str.split_once(' ');
        if temp.is_some(){
            let (command, arguments) = temp.unwrap();
            if command == "help"{
                Terminal::help();
            }
            else if command == "say"{
                Terminal::say(arguments.parse().unwrap());
            }
            else{
                Terminal::wrong_command(command.parse().unwrap());
            }
        }
        else{
            if(str == "help"){
                Terminal::help();
            }
        }
    }

}

pub struct Terminal{

}

impl Terminal{
    pub const fn new() -> Terminal {return Terminal{}}

    pub fn help(){
        emergency_message("SnakeOS help:\n");
        emergency_message("Commands:\n");
        emergency_message("help (this command): displays commands\n");
        emergency_message("say (in double quo marks) message: displays a message\n");
    }

    pub fn say(message:String){
        emergency_message(&*message);
        emergency_message("\n");
    }

    pub fn wrong_command(command:String){
        emergency_message("Wrong command:");
        emergency_message(command.as_str());
        emergency_message("\n");
    }
}

pub static TERMINAL: Mutex<Terminal> = Mutex::new(Terminal::new());

pub static COMMAND_PARSER: Mutex<CommandParser> = Mutex::new(CommandParser::new());