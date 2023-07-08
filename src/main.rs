use colored::Colorize;
use rand::Rng;
use std::env;

// command line password generator tool      
// syntax: passGen -numberOfChars -y/n numbers -y/n specialChars
fn main() {    
    let args: Vec<String> = env::args().collect();

    if args[1].eq(&"-h".to_string()){
        println!("{}\n{}\n{}",
            "Thank you for using PassGen!".truecolor(28, 49, 171),
            "Usage: pass_gen [length of password] [y/n use numbers] [y/n use special chars] [y/n use caps]".truecolor(89, 111, 98).bold(), 
            "Leave all fields empty for deafult: all active.".truecolor(28, 49, 171))
    }else{
        go(args);
    }
}


fn go(args:Vec<String>){
    let yes : String = String::from("y");

    let pw_length = &args[1].parse::<u32>().unwrap();
    let numbers = &args[2].eq(&yes).to_owned();
    let special = &args[3].eq(&yes).to_owned();
    let caps = &args[4].eq(&yes).to_owned();

    //println!("nChars: {pw_length}\nnumbers: {numbers}\nspecialChars: {special}\ncaps: {caps}");

    let generated:String = generator(*pw_length, *numbers, *special, *caps);
    
    println!("{}", generated.truecolor(102, 31, 179));
}


fn generator(l:u32, nb:bool, s:bool, c:bool) -> String {
    let mut pw:String = "".to_owned(); 
    let mut abc:String = "abcdefghijklmnopqrstuvwxyz".to_owned();
    let caps:&str = "ABCDEFGHILMNOPQRTSTUVWXYZ";
    let special:&str = "|!?£$%&/()[]{}=^<>+-";
    let nums:&str = "1234567890";

    if nb { abc.push_str(nums)}

    if s { abc.push_str(special)}

    if c { abc.push_str(caps)}

    for _ in 0..l {
        let random = rand::thread_rng().gen_range(0..abc.chars().count());
        
        let nth = abc.chars().nth(random).unwrap();

        pw.push_str(&nth.to_string());
    }

    pw
}