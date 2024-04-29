fn main(){
    //arrays ,maps,strings
    let sentence=String::from("my name is mohak");
    let first_word=get_first_word(sentence);

    let n=1000;
    for i in 0..n{
        println!("Hello world,! {}",i);
    }

    println!("First word is : {}",first_word);

}

fn get_first_word(sentence:String)->String{
    let mut ans: String =String::from("");
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }

    return ans;

}