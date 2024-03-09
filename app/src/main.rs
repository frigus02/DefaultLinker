use std::io::{Read, Write};

fn main() {
    let Some(origin) = std::env::args().nth(1) else {
        println!("{{\"exit\":2}}");
        std::process::exit(0);
    };
    if origin != "chrome-extension://iajapgoidjefnaekmbeinpnodonpdnaj/" {
        println!("{{\"exit\":3}}");
        std::process::exit(0);
    }

    let buf_msg = {
        let mut stdin = std::io::stdin().lock();
        let mut buf_msg_len: [u8; 4] = [0, 0, 0, 0];
        stdin.read_exact(&mut buf_msg_len[..]).unwrap();
        let msg_len = u32::from_ne_bytes(buf_msg_len);
        let mut buf_msg = String::with_capacity(msg_len as usize);
        stdin
            .take(msg_len as u64)
            .read_to_string(&mut buf_msg)
            .unwrap();
        buf_msg
    };

    let msg = serde_json::from_str::<serde_json::Value>(&buf_msg).unwrap();
    let url = msg["url"].as_str().unwrap();
    let result = if webbrowser::open(url).is_ok() { 0 } else { 1 };

    let res = format!("{{\"exit\":{result}}}");
    let res_len: u32 = res.len() as u32;
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(&res_len.to_ne_bytes()).unwrap();
    stdout.write_all(res.as_bytes()).unwrap();
}
