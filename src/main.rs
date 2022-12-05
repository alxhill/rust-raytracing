use std::io::{BufWriter, stdout};

fn main() {
    let stdout = stdout();
    let msg = String::from("Hello world!");
    let len = msg.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(msg.as_bytes(), len, &mut writer).unwrap();
}
