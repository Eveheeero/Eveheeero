use git2::{build::CheckoutBuilder, Repository};
use std::{path::PathBuf, process::Command};

pub fn download_github_archive(
    basedir: &PathBuf,
    target_name: &str,
    user: &str,
    repository: &str,
    tag: &Option<&str>,
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

    download_url_tar_gz::<false, true>(basedir, target_name, &url)
}

pub fn download_github(
    basedir: &PathBuf,
    target_name: &str,
    user: &str,
    repository: &str,
    tag: Option<&str>,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Some(tag) = tag {
        let url = format!("https://github.com/{user}/{repository}.git");
        let mut checkout = CheckoutBuilder::new();
        checkout.their_label(tag);
        checkout.force();

        Repository::clone_recurse(&url, basedir.join(target_name))?
            .checkout_head(Some(&mut checkout))?;

        Ok(basedir.join(target_name))
    } else {
        let lastest = get_github_lastest_releases(user, repository)?;
        let url = format!("https://github.com/{user}/{repository}.git");
        let mut checkout = CheckoutBuilder::new();
        checkout.their_label(lastest.as_str());
        checkout.force();

        Repository::clone_recurse(&url, basedir.join(target_name))?
            .checkout_head(Some(&mut checkout))?;

        Ok(basedir.join(target_name))
    }
}

pub fn download_url_tar_gz<const CURL: bool, const STRIP: bool>(
    basedir: &PathBuf,
    target_name: &str,
    url: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // 다운로드
    let archived;
    if CURL {
        archived = bytes::Bytes::from(Command::new("curl").arg("-L").arg(url).output()?.stdout);
    } else {
        archived = reqwest::blocking::Client::default()
            .get(url)
            .header("User-Agent", "hyperscan")
            .send()?
            .bytes()?;
    }

    // 압축 해제
    let archived = flate2::read::GzDecoder::new(&archived[..]);
    let mut archive = tar::Archive::new(archived);
    archive.unpack(basedir.join(target_name))?;

    // 이름 변경
    if STRIP {
        std::fs::rename(
            std::fs::read_dir(basedir.join(target_name))?
                .next()
                .unwrap()?
                .path(),
            basedir.join("temp"),
        )?;
        std::fs::rename(basedir.join("temp"), basedir.join(target_name))?;
    }

    Ok(basedir.join(target_name))
}

pub fn download_url_zip<const CURL: bool, const STRIP: bool>(
    basedir: &PathBuf,
    target_name: &str,
    url: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // 다운로드
    let archived;
    if CURL {
        archived = bytes::Bytes::from(Command::new("curl").arg("-L").arg(url).output()?.stdout);
    } else {
        archived = reqwest::blocking::Client::default()
            .get(url)
            .header("User-Agent", "hyperscan")
            .send()?
            .bytes()?;
    }

    // 압축 해제
    let mut archived = zip::ZipArchive::new(std::io::Cursor::new(archived))?;
    archived.extract(basedir.join(target_name))?;

    // 이름 변경
    if STRIP {
        std::fs::rename(
            std::fs::read_dir(basedir.join(target_name))?
                .next()
                .unwrap()?
                .path(),
            basedir.join("temp"),
        )?;
        std::fs::rename(basedir.join("temp"), basedir.join(target_name))?;
    }

    Ok(basedir.join(target_name))
}

pub fn get_github_lastest_releases(
    user: &str,
    repository: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.github.com/repos/{user}/{repository}/releases",
        user = user,
        repository = repository
    );
    let releases = reqwest::blocking::Client::default()
        .get(&url)
        .header("User-Agent", "hyperscan")
        .send()?
        .json::<Vec<serde_json::Value>>()?;

    Ok(releases[0]["tag_name"].as_str().unwrap().to_string())
}
