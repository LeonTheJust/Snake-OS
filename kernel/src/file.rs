use alloc::string::String;

pub struct File{
    pub name : [u8;16],
    pub name_len : usize,
    pub(crate) contents : [u8;256],
    pub content_len : usize,
}

impl File{
    pub fn new(name_str : String) -> File {
        let mut name_len : usize;
        let mut name : [u8;16] = [0;16];
        if(name_str.len() <= 16 && !name_str.is_empty()){
            let bytes: &[u8] = name_str.as_bytes();
            name[..bytes.len()].copy_from_slice(bytes);
            name_len = name_str.len();
        }
        else{
            let temp = "untitled";
            let bytes: &[u8] = temp.as_bytes();
            name[..bytes.len()].copy_from_slice(bytes);
            name_len = temp.len();
        }
        let contents = [0;256];
        let content_len = 0;
        return File{name,name_len,contents,content_len};
    }

    pub fn write(&mut self,data : String){
        let mut i:usize = 0;
        if(data.len() > 256){
            return;
        }
        for c in data.chars(){
            self.contents[i] = c as u8;
            i = i + 1;
        }
        self.contents[i] = u8::try_from('\0').unwrap();
        self.content_len = i;
    }

    pub fn rename(&mut self,name : String){
        if(name.len() > 16 || name.is_empty()){
            let temp = "untitled";
            let bytes = temp.as_bytes();
            let mut new_name : [u8;16] = [0;16];
            new_name[..bytes.len()].copy_from_slice(bytes);
            self.name = new_name;
            self.name_len = temp.len();
            return;
        }
        let mut i:usize = 0;
        if(name.len() > 16){
            return;
        }
        for c in name.chars(){
            self.name[i] = c as u8;
            i = i + 1;
        }
        self.name_len = i;
    }

    pub fn append(&mut self,data : String){
        let mut i:usize = self.content_len;
        if(data.len() > 256 - i){
            return;
        }
        for c in data.chars(){
            self.contents[i] = c as u8;
            i = i + 1;
        }
        self.content_len = i;
    }

    pub fn clear(&mut self){
        self.contents = [0;256];
        self.content_len = 0;
    }

}