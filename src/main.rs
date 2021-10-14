use std;

fn main() {
    let path = std::env::args().last().expect("no path given");
    let contents = std::fs::read_to_string(path).expect("Error reading file from path!");
    render(&contents)
}

pub fn render(img: &str) {
    for c in img.chars() {
        match c {
            '0'=>print!(" "),
            '1'=>print!("█"),
            '\n'=>print!("\n"),
            _=>panic!("unkown char '{}'!", c)
        }
    }
}