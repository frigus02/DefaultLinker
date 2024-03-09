use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize)]
struct Request {
    url: String,
}

#[derive(Debug, Serialize)]
struct Response {
    result: u8,
}

type BoxError = Box<dyn std::error::Error>;

fn read_request() -> Result<Request, BoxError> {
    let mut stdin = std::io::stdin().lock();
    let mut buf_msg_len: [u8; 4] = [0, 0, 0, 0];
    stdin.read_exact(&mut buf_msg_len[..])?;
    let msg_len = u32::from_ne_bytes(buf_msg_len);
    let mut buf_msg = String::with_capacity(msg_len as usize);
    stdin.take(msg_len as u64).read_to_string(&mut buf_msg)?;
    let req = serde_json::from_str::<Request>(&buf_msg)?;
    Ok(req)
}

fn write_response(res: Response) -> Result<(), BoxError> {
    let res = serde_json::to_vec(&res)?;
    let res_len: u32 = res.len() as u32;
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(&res_len.to_ne_bytes())?;
    stdout.write_all(&res)?;
    Ok(())
}

fn main() -> Result<(), BoxError> {
    let origin = std::env::args()
        .nth(1)
        .ok_or("1st arg must be caller origin")?;
    if origin != "chrome-extension://iajapgoidjefnaekmbeinpnodonpdnaj/" {
        Err("Invalid caller origin")?;
    }

    let req = read_request()?;
    let result = if webbrowser::open(&req.url).is_ok() {
        0
    } else {
        1
    };
    let res = Response { result };
    write_response(res)?;

    Ok(())
}
