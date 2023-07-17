

pub fn decode_bytes(line: &String) -> String {

    let mut cnt = 0;
    let mut proto: char;
    let mut command = String::new();
    for each_byte in line.chars() {
        if cnt == 0 {
            proto = each_byte;
        } else {
            command.push(each_byte);
        }
        cnt += 1;
    }
    //println!("command: {}", command);

    command = command.to_lowercase();

    match command.as_str() {
        "ping" => "+PONG".to_string(),
        _ => "*1\r\n$4\r\n+PONG\r\n".to_string()
    }
}