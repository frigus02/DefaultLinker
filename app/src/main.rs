use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Serialize)]
struct Manifest {
    name: &'static str,
    description: &'static str,
    path: String,
    #[serde(rename = "type")]
    type_: &'static str,
    allowed_origins: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Request {
    url: String,
}

#[derive(Debug, Serialize)]
struct Response {
    result: u8,
}

type BoxError = Box<dyn std::error::Error>;

fn maybe_install() -> Result<bool, BoxError> {
    let mut args = std::env::args();
    match args.nth(1).as_deref() {
        Some("--install") => {}
        _ => return Ok(false),
    };
    let Some(extension_id) = args.next() else {
        return Err("expected extension id after --install flag".into());
    };

    let path = std::env::current_exe()?;
    let manifest = Manifest {
        name: "me.kuehle.open_app_links_in_default_browser",
        description: "Open external Chrome app links in the default browser",
        path: path.to_str().ok_or("exe path is not valid utf8")?.into(),
        type_: "stdio",
        allowed_origins: vec![format!("chrome-extension://{extension_id}/")],
    };

    #[cfg(target_os = "linux")]
    {
        let manifest_path = PathBuf::from(format!(
            "~/.config/google-chrome/NativeMessagingHosts/{}.json",
            manifest.name,
        ));
        std::fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;
        println!("Wrote manifest to {}", manifest_path.display());
    }

    #[cfg(target_os = "macos")]
    {
        let manifest_path = PathBuf::from(format!(
            "~/Library/Application Support/Google/Chrome/NativeMessagingHosts/{}.json",
            manifest.name,
        ));
        std::fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;
        println!("Wrote manifest to {}", manifest_path.display());
    }

    #[cfg(target_os = "windows")]
    {
        let mut manifest_path = path.clone();
        manifest_path.set_file_name("manifest.json");
        std::fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;
        println!("Wrote manifest to {}", manifest_path.display());
        println!("Please run the following command to register the manifest:");
        println!(
            "  REG ADD \"HKCU\\Software\\Google\\Chrome\\NativeMessagingHosts\\{}\" /ve /t REG_SZ /d \"{}\" /f",
            manifest.name,
            manifest_path.display(),
        );
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    return Err("unsupported operating system".into());

    Ok(true)
}

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
    if maybe_install()? {
        return Ok(());
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
