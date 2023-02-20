use std::{io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = std::env::var("OUT_DIR")?.parse::<PathBuf>()?;

    download_hyperscan(&target_dir, None);

    Ok(())
}

// fn download_boost(basedir: &PathBuf, version: Option<String>) -> PathBuf {}

fn download_hyperscan(basedir: &PathBuf, version: Option<String>) -> PathBuf {
    let mut hyperscan_path = basedir.join("hyperscan");

    match download_hyperscan_zip(basedir, &version) {
        Ok(obj) => return obj,
        Err(_) => {}
    }

    hyperscan_path
}

fn download_hyperscan_zip(
    basedir: &PathBuf,
    version: &Option<String>,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    /* 압축파일 다운로드 및 저장 */
    {
        let archived;
        if let Some(version) = version {
            archived = reqwest::blocking::Client::default()
                .get(format!(
                    "https://api.github.com/repos/intel/hyperscan/tarball/{version}"
                ))
                .header("User-Agent", "hyperscan")
                .send()?
                .bytes()?;
        } else {
            archived = reqwest::blocking::Client::default()
                .get("https://api.github.com/repos/intel/hyperscan/tarball/")
                .header("User-Agent", "hyperscan")
                .send()?
                .bytes()?;
        }
        let mut writer = std::fs::File::create(basedir.join("hyperscan.tar.gz"))?;
        writer.write_all(&archived)?;
    }

    /* 압축해제 */
    {
        let mut archive = tar::Archive::new(std::fs::File::open(basedir.join("hyperscan.tar.gz"))?);
        archive.unpack(basedir.join("hyperscan"))?;
    }

    return Ok(basedir.join("hyperscan"));
}
