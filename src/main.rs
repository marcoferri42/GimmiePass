use std::env;
extern crate colored;
use colored::Colorize;
extern crate rand;
use rand::Rng;
extern crate clipboard;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;


// command line password generator tool      
// syntax: passGen -numberOfChars -y/n numbers -y/n specialChars
fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        args.push(String::from("y"));
        args.push(String::from("y"));
        args.push(String::from("y"));
    }

    if args.len() < 5 || args[1] == "-h" {
        print_help();
    } else {
        go(args);
    }
}

fn print_help() {
    let help = format!(
        "{}\n{}\n{}",
        "Thank you for using GimmiePass!".truecolor(89, 111, 98).bold(),
        "Usage: pass_gen [length of password] [y/n use numbers] [y/n use special chars] [y/n use caps]".white().bold(),
        "Leave all fields empty for default: all active.".truecolor(89, 111, 98).bold(),
    );
    println!("{}", help);
}


fn go(args:Vec<String>){
    // TODO input type check

    let yes : String = String::from("y");

    let pw_length = &args[1].parse::<u32>().unwrap();
    let numbers = &args[2].eq(&yes).to_owned();
    let special = &args[3].eq(&yes).to_owned();
    let caps = &args[4].eq(&yes).to_owned();

    //println!("nChars: {pw_length}\nnumbers: {numbers}\nspecialChars: {special}\ncaps: {caps}");

    let generated:String = generator(*pw_length, *numbers, *special, *caps);
    
    println!("{}", generated.white().bold());
    
    if pw_length <= &1000 {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(generated.to_owned()).unwrap();
        let _ = ctx.get_contents(); // apparently ctx lib works only if this is here
        
        println!("{} password has been copied to clipboard.", "☑".green());
    } else {
        println!("{} password was too long to be safely copied to clipboard.", "⚠".yellow());
    }
    
}


fn generator(l:u32, nb:bool, s:bool, c:bool) -> String {
    let mut pw = String::new();
    let mut abc = "abcdefghijklmnopqrstuvwxyz".to_owned();
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