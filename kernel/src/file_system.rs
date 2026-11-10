use alloc::string::String;
use spin::Mutex;
use crate::file::File;
use crate::serial::emergency_message;

pub struct FileSystem{
    files : [Option<File>; 16],
    file_count : usize,
}

impl FileSystem{
    pub const fn new() -> FileSystem{
        let files = [const { None }; 16];
        let file_count = 0;
        FileSystem{files, file_count}
    }

    pub fn create(&mut self, name : String){
        if(self.file_count == 16){
            emergency_message("File system fullllll");
        }
        else{
            for i in 0..16{
                if self.files[i].is_none() {
                    let file = File::new(name.clone());
                    self.files[i] = Some(file);
                    break;
                }
            }
            self.file_count+=1;
        }
        emergency_message(name.as_str());
        emergency_message("test\n");
    }

    pub fn find(&self, name : String) -> Option<usize>{
        for i in 0..16{
            let Some(file) = &self.files[i] else { continue; };
            let temp = core::str::from_utf8(&file.name[..file.name_len]).unwrap();
            if(temp == name){
                return Some(i);
            }
        }
        return None;
    }

    pub fn delete(&mut self, name : String){
        let index = self.find(name.clone()).unwrap();
        self.files[index] = None;
        self.file_count-=1;
    }

    pub fn read(&mut self, name : String) -> Option<String>{
        let index_temp = self.find(name.clone());
        if(index_temp != None){
            let index = index_temp.unwrap();
            let file = self.files[index].as_mut().unwrap();
            let contents = core::str::from_utf8(&file.contents[0..file.content_len]).unwrap();
            return Some(contents.parse().unwrap());
        }
        emergency_message("not found");
        return None;
    }

    pub fn list(&mut self) -> String{
        let mut list: String = String::new();
        for i in 0..16{
            match &self.files[i] {
                Some(file) =>{
                let file_name = core::str::from_utf8(&file.name[..file.name_len]).unwrap();
                list.push_str(file_name);
                list.push_str(" ");}
                None =>{}
            }
        }
        return list;
    }

    pub fn write(&mut self, name : String, contents : String){
        let index_temp = self.find(name.clone());
        if index_temp != None{
            let index = index_temp.unwrap();
            let file = self.files[index].as_mut().unwrap();
            let bytes = contents.as_bytes();
            let mut content : [u8;256] = [0;256];
            content[..bytes.len()].copy_from_slice(bytes);
            file.contents = content;
            file.write(contents);
        }
        else{
            emergency_message("No file found to write: ");
            emergency_message(name.as_str());
            emergency_message("yeah");
        }
    }
}

pub static FILESYSTEM : Mutex<FileSystem> = Mutex::new(FileSystem::new());