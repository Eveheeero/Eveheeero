use std::path::PathBuf;

use git2::{build::CheckoutBuilder, Repository};
use once_cell::sync::Lazy;

static BASEDIR: Lazy<PathBuf> = Lazy::new(|| {
    std::env::var("OUT_DIR")
        .expect("Not In Build!")
        .parse::<PathBuf>()
        .unwrap()
});

fn main() -> Result<(), Box<dyn std::error::Error>> {
    download_hyperscan(None);
    download_github("boost", "boostorg", "boost", "boost-1.81.0")?;
    Ok(())
}

// fn download_boost(basedir: &PathBuf, version: Option<String>) -> PathBuf {}

fn download_hyperscan(version: Option<String>) -> PathBuf {
    match download_github_zip("hyperscan", "intel", "hyperscan", &version) {
        Ok(obj) => return obj,
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    todo!("download_hyperscan_git")
}

fn download_github_zip(
    target_name: &str,
    user: &str,
    repository: &str,
    tag: &Option<String>,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // 링크 생성
    let mut url = format!(
        "https://api.github.com/repos/{user}/{repository}/tarball",
        user = user,
        repository = repository
    );
    if let Some(tag) = tag {
        url = format!("{url}/{tag}", url = url, tag = tag);
    }

    // 다운로드
    let archived = reqwest::blocking::Client::default()
        .get(url)
        .header("User-Agent", "hyperscan")
        .send()?
        .bytes()?;

    // 압축 해제
    let archived = flate2::read::GzDecoder::new(&archived[..]);
    let mut archive = tar::Archive::new(archived);
    archive.unpack(BASEDIR.join(target_name))?;

    // 이름 변경
    std::fs::rename(
        std::fs::read_dir(BASEDIR.join(target_name))?
            .next()
            .unwrap()?
            .path(),
        BASEDIR.join("temp"),
    )?;
    std::fs::rename(BASEDIR.join("temp"), BASEDIR.join(target_name))?;

    Ok(BASEDIR.join(target_name))
}

fn download_github(
    target_name: &str,
    user: &str,
    repository: &str,
    tag: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let url = format!("https://github.com/{user}/{repository}.git");
    let mut checkout = CheckoutBuilder::new();
    checkout.their_label(tag);
    checkout.force();

    Repository::clone_recurse(&url, BASEDIR.join(target_name))?
        .checkout_head(Some(&mut checkout))?;

    Ok(BASEDIR.join(target_name))
}
