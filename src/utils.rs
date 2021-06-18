use crate::GBDResult;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::IntoUrl;
use serde_json::Value;
use std::{cmp::min, fs::File, io::Write};

pub fn humanize_bytes(bytes: u64) -> String {
    let values = ["bytes", "KB", "MB", "GB", "TB"];
    let pair = values
        .iter()
        .enumerate()
        .take_while(|x| bytes as usize / (1000_usize).pow(x.0 as u32) > 10)
        .last();
    if let Some((i, unit)) = pair {
        format!("{} {}", bytes as usize / (1000_usize).pow(i as u32), unit)
    } else {
        format!("{} {}", bytes, values[0])
    }
}

pub fn trim_newline(mut s: String) -> String {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    s
}

pub fn compare_two_vector<T>(vec1: &[T], vec2: &[T]) -> Option<Vec<T>>
where
    T: PartialEq + Clone,
{
    let mut result: Vec<T> = Vec::new();
    for v1 in vec1 {
        for v2 in vec2 {
            if v1 == v2 {
                result.push(v1.clone());
            }
        }
    }
    if !result.is_empty() {
        Some(result)
    } else {
        None
    }
}

pub fn sanitize_str_to_string(string: &Value) -> String {
    string.to_string().replace('"', "")
}

pub async fn download_file_from_url<T>(url: T, name: &str) -> GBDResult<()>
where
    T: IntoUrl,
{
    let mut resp = reqwest::get(url).await?;
    resp.content_length();
    let mut f = File::create(name)?;
    let mut downloaded = 0;
    let total_size = resp.content_length().unwrap();
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .progress_chars("#>-"));
    while let Some(chunk) = resp.chunk().await? {
        let new = min(downloaded + chunk.len() as u64, total_size);
        downloaded = new;
        pb.set_position(new);
        f.write_all(&chunk[..])?;
    }
    Ok(())
}
