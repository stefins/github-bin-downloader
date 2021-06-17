use crate::ghapi::Release;

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

pub fn compare_two_vector(vec1: &[Release], vec2: &[Release]) -> Option<Vec<Release>> {
    let mut releases: Vec<Release> = Vec::new();
    for v1 in vec1 {
        for v2 in vec2 {
            if v1 == v2 {
                releases.push(v1.clone());
            }
        }
    }
    if !releases.is_empty() {
        Some(releases)
    } else {
        None
    }
}
