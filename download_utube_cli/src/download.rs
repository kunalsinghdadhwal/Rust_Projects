use crate::error;
use reqwest;
use serde_json;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Error;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn auto(
    prefix: &str,
    debug: bool,
    apiurl: &str,
    path: &str,
    url: &str,
    quality: &str,
    codec: &str,
    ttwatermark: bool,
    audioformat: &str,
    dublang: bool,
    fullaudio: bool,
    mute: bool,
) {
    println!("{prefix} getting stream URL for {}", url);

    let mut getstream_body = HashMap::new();
    getstream_body.insert("url", url);
    getstream_body.insert("vCodec", codec);
    getstream_body.insert("vQuality", quality);
    getstream_body.insert("aFormat", audioformat);

    let inttwm = &ttwatermark.to_string();
    let ifa = &fullaudio.to_string();
    let iam = &mute.to_string();
    let idl = &dublang.to_string();
    if ttwatermark == true {
        getstream_body.insert("isNoTTWatermark", inttwm);
    }
    if fullaudio == true {
        getstream_body.insert("isTTFullAudio", ifa);
    }
    if mute == true {
        getstream_body.insert("isAudioMuted", iam);
    }
    if dublang == true {
        getstream_body.insert("dubLang", idl);
    }

    let getstream_url = &format!("https://{apiurl}/api/json");

    if debug {
        println!(" ");
        println!("{prefix} {}", "====[ debug ]====");
        println!("{prefix} get stream url request url:");
        println!("{prefix} {}", getstream_url);
        println!("{prefix} get stream url request body:");
        println!(
            "{prefix} {}",
            serde_json::to_string(&getstream_body).unwrap()
        );
        println!("{prefix} {}", "====[ debug ]====");
        println!(" ");
    }

    getstream(prefix, &getstream_url, getstream_body, path);
}

pub fn audio(
    prefix: &str,
    debug: bool,
    apiurl: &str,
    path: &str,
    url: &str,
    quality: &str,
    codec: &str,
    ttwatermark: bool,
    audioformat: &str,
    dublang: bool,
    fullaudio: bool,
    mute: bool,
) {
    println!("{prefix} getting stream URL for {}...", url);

    let mut getstream_body = HashMap::new();
    getstream_body.insert("isAudioOnly", "true");
    getstream_body.insert("url", url);
    getstream_body.insert("vCodec", codec);
    getstream_body.insert("vQuality", quality);
    getstream_body.insert("aFormat", audioformat);
    let inttwm = &ttwatermark.to_string();
    let ifa = &fullaudio.to_string();
    let iam = &mute.to_string();
    let idl = &dublang.to_string();
    if ttwatermark == true {
        getstream_body.insert("isNoTTWatermark", inttwm);
    }
    if fullaudio == true {
        getstream_body.insert("isTTFullAudio", ifa);
    }
    if mute == true {
        getstream_body.insert("isAudioMuted", iam);
    }
    if dublang == true {
        getstream_body.insert("dubLang", idl);
    }

    let getstream_url = &format!("https://{apiurl}/api/json");
    if debug {
        println!(" ");
        println!("{prefix} {}", "====[ debug ]====");
        println!("{prefix} get stream url request url:");
        println!("{prefix} {}", getstream_url);
        println!("{prefix} get stream url request body:");
        println!(
            "{prefix} {}",
            serde_json::to_string(&getstream_body).unwrap()
        );
        println!("{prefix} {}", "====[ debug ]====");
        println!(" ");
    }

    getstream(prefix, &getstream_url, getstream_body, path);
}

#[tokio::main]
async fn getstream(prefix: &str, url: &str, body: HashMap<&str, &str>, path: &str) {
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .json(&body)
        .send()
        .await;

    let formatted_res = response
        .expect("Method not found in `Result<Response, Error>`")
        .text()
        .await
        .unwrap();
    let fmtd_res2: Value = serde_json::from_str(&formatted_res).unwrap();

    if fmtd_res2.get("status").unwrap() == "stream" {
        let streamurl = fmtd_res2.get("url").unwrap().to_string();

        let streamurl: &str = &streamurl[1..streamurl.len() - 1];

        let idk: Result<()> = downloadfromstream(prefix, &streamurl.to_string(), path).await;

        println!("{:?}", idk);
    } else {
        error::create_end(
            &format!(
                "{} failed to get stream url. {}",
                prefix,
                fmtd_res2.get("text").unwrap()
            )
            .as_str(),
        );
    }
}

async fn downloadfromstream(prefix: &str, url: &str, path: &str) -> Result<()> {
    println!("{} got stream url. starting download...", prefix);
    let response = reqwest::get(url.to_string()).await?;
    let file1 = response
        .headers()
        .get("Content-Disposition")
        .unwrap()
        .to_str()
        .ok();
    let file2 = file1.unwrap().strip_prefix("attachment; filename=\"");
    let file3 = file2.unwrap().strip_prefix("\"").unwrap();

    let full_path = format!("{}/{}", path, file3);

    let mut file = std::fs::File::create(format!("{path}/{file3}"))?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    println!("{} completed download. saved as {}", prefix, full_path);

    Ok(())
}

