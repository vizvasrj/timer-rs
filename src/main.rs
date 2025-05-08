use std::{thread, time::Duration};
fn main() {
    
    let args: String = std::env::args().skip(1).collect();
    let mut number: Vec<u32> = Vec::new();
    let mut d = Duration::new(0, 0);
    let mut number_string = String::new();
    let mut time_string = String::new();
    args.split("").for_each(|s|  {
        s.parse::<u32>().ok().map(|n| {
            number_string.push_str(s);
            if number.len() > 0 {
                for i in 0..number.len() {
                    number[i] = number[i] * 10;
                }
            }
            number.push(n);
        }).or_else(|| {
            if s == "m" {
                time_string.push_str("minutes");
                d = Duration::new(60, 0);
            } else if s == "s" {
                time_string.push_str("seconds");
                d = Duration::new(1, 0);
            } else if s == "." {

            }
            None
        });
    });
    if d == Duration::new(0, 0) {
        println!("Please provide a time unit (m/s)");
        return;
    }
    let t = format!("sleep for {} {}", number_string, time_string);
    thread::sleep(d * number.iter().sum::<u32>());
    println!("sleep done");
    thread::spawn(||{
        std::process::Command::new("paplay")
            .arg("/usr/share/sounds/freedesktop/stereo/complete.oga")
            .output()
            .expect("failed to execute process");  
    });
    std::process::Command::new("zenity")
        .arg("--info")
        .arg(format!("--text={t} is done", t=t))
        .output()
        .expect("failed to execute process");

    
}
