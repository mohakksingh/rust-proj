use std::fs;

fn main(){
    let res= read_from_file_unsafe("example.txt".to_string());
}

fn read_from_file_unsafe(file_content:String)->Result<String,String>{
    let res=fs::read_to_string("example.txt");

    if let Ok(content)=res{
        return Ok(content);
    }else{
        return Err("Error reading file".to_string());
    }
}