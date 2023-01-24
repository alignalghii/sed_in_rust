use std::io;

fn main() {
    let input_stream_channel = io::stdin();
    loop {
        let mut line = String::new();
        if let Ok(n) = input_stream_channel.read_line(&mut line) {
            if n > 0 {
                trim_newline(&mut line);
                println!("{line}");
            } else {
                break;
            }
        }
    }
}

// @credit to https://stackoverflow.com/q/37888042
fn trim_newline(s: &mut String) {
    if let Some(last) = s.pop() {
        if last != '\n' {
            s.push(last);
        }
    }
}
