use alloc::string::{String, ToString};
use bootloader_api::info::FrameBuffer;
use spin::Mutex;
use crate::serial::emergency_message;
use crate::text_render;

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

    pub fn lex(&mut self, x: i32, y: i32, frame_buffer: &mut FrameBuffer){
        let str = self.command_buffer.current_command();
        let temp  = str.split_once(' ');
        if temp.is_some(){
            let (command, arguments) = temp.unwrap();
            if command == "help"{
                Terminal::help(frame_buffer,x,y);
            }
            else if command == "say"{
                Terminal::say(frame_buffer,x,y ,arguments.parse().unwrap());
            }
            else{
                Terminal::wrong_command(frame_buffer,x,y ,command.parse().unwrap());
            }
        }
        else{
            if(str == "help"){
                Terminal::help(frame_buffer,x,y);
            }
        }
    }

}

pub struct Terminal{

}

impl Terminal{
    pub const fn new() -> Terminal {return Terminal{}}

    pub fn help(frame_buffer:&mut FrameBuffer, mut x:i32, y :i32){
        let str = "SnakeOS help:\nSnakeOS help:\nhelp (this command): displays commands\nsay (in double quo marks) message: displays a message\n";
        emergency_message(str);
        for char in str.chars(){
            text_render::draw_character(frame_buffer,y as usize, x as usize, char);
            x = x + 8;
        }
    }

    pub fn say(frame_buffer:&mut FrameBuffer, mut x:i32, y :i32, message:String){
        emergency_message(&*message);
        emergency_message("\n");
        for char in message.chars(){
            text_render::draw_character(frame_buffer,y as usize, x as usize, char);
            x = x + 8;
        }
        text_render::draw_character(frame_buffer,y as usize, x as usize, '\n');

    }

    pub fn wrong_command(frame_buffer:&mut FrameBuffer, mut x:i32, y :i32, command:String){
        emergency_message("Wrong command:");
        emergency_message(command.as_str());
        emergency_message("\n");
        for chat in "Wrong command:".chars(){
            text_render::draw_character(frame_buffer,y as usize, x as usize, chat);
            x = x + 8;
        }
        for char in command.chars(){
            text_render::draw_character(frame_buffer,y as usize, x as usize, char);
            x = x + 8;
        }
        text_render::draw_character(frame_buffer,y as usize, x as usize, '\n');
    }
}

pub static TERMINAL: Mutex<Terminal> = Mutex::new(Terminal::new());

pub static COMMAND_PARSER: Mutex<CommandParser> = Mutex::new(CommandParser::new());