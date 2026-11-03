use spin::Mutex;
use crate::serial::{emergency_message, u64_to_str};

pub struct KeyboardQueue {
    queue : [u8;64],
    head : usize,
    tail : usize,
    count : u8,
    decode_table: [char; 128]
}

impl KeyboardQueue {
    pub const fn new() -> KeyboardQueue {
        let mut queue = [0u8;64];
        let mut decode_table : [char;128] = ['\0';128];
        decode_table[2] = '1';
        decode_table[3] = '2';
        decode_table[4] = '3';
        decode_table[5] = '4';
        decode_table[6] = '5';
        decode_table[7] = '6';
        decode_table[8] = '7';
        decode_table[9] = '8';
        decode_table[10] = '9';
        decode_table[11] = '0';
        decode_table[12] = '-';
        decode_table[13] = '=';
        decode_table[16] = 'q';
        decode_table[17] = 'w';
        decode_table[18] = 'e';
        decode_table[19] = 'r';
        decode_table[20] = 't';
        decode_table[21] = 'y';
        decode_table[22] = 'u';
        decode_table[23] = 'i';
        decode_table[24] = 'o';
        decode_table[25] = 'p';
        decode_table[26] = '[';
        decode_table[27] = ']';
        decode_table[28] = '\\';
        decode_table[30] = 'a';
        decode_table[31] = 's';
        decode_table[32] = 'd';
        decode_table[33] = 'f';
        decode_table[34] = 'g';
        decode_table[35] = 'h';
        decode_table[36] = 'j';
        decode_table[37] = 'k';
        decode_table[38] = 'l';
        decode_table[39] = ';';
        decode_table[40] = '\'';
        decode_table[41] = '`';
        decode_table[43] = '\\';
        decode_table[44] = 'z';
        decode_table[45] = 'x';
        decode_table[46] = 'c';
        decode_table[47] = 'v';
        decode_table[48] = 'b';
        decode_table[49] = 'n';
        decode_table[50] = 'm';
        decode_table[51] = ',';
        decode_table[52] = '.';
        decode_table[53] = '/';
        decode_table[57] = ' ';
        return KeyboardQueue { queue , head : 0, tail : 0, count : 0, decode_table };
    }

    pub fn push(&mut self, ch : u8) {
        if self.count < 63 && self.head < 62{
            if self.head != self.tail {
                self.queue[self.head] = ch;
                self.head += 1;
                self.count+=1;

            }
            else{
                self.head = 1;
                self.tail = 0;
                self.queue[self.tail] = ch;
                self.count+=1;
            }
        }
        else{
            if self.tail == 0|| self.count >= 64 {emergency_message("queue full, element NOT pushed");}
            else {
                //emergency_message("AAA\n\n\n");
                let mut temp = [0u8;64];
                for i in self.tail..self.head{
                    temp[i - self.tail] = self.queue[i];
                }
                let mut buffer = [0u8;20];
                self.head = self.head - self.tail;
                self.tail = 0;
                for i in 0..self.head{
                    self.queue[i] = temp[i];
                }
                self.queue[self.head] = ch;
                self.count = (self.head + 1) as u8;
                emergency_message("AAA\n\n\n");
            }
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.count > 0 {
            self.tail += 1;
            self.count -=1;
            return Some(self.queue[self.tail - 1]);
        }
        else{
            //emergency_message("queue empty, no element popped");
            return None;
        }
    }
    pub fn decode(&mut self,code :u8) -> Option<char> {
        return Some(self.decode_table[code as usize]);
    }
}



pub(crate) static KEYBOARD_QUEUE : Mutex<KeyboardQueue> = Mutex::new(KeyboardQueue::new());