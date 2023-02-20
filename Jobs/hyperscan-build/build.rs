use std::{path::PathBuf, process::Command};

use git2::{build::CheckoutBuilder, Repository};
use once_cell::sync::Lazy;

static BASEDIR: Lazy<PathBuf> = Lazy::new(|| {
    std::env::var("OUT_DIR")
        .expect("Not In Build!")
        .parse::<PathBuf>()
        .unwrap()
});

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let boost = download_boost(Some("boost-1.81.0"));
    let hyperscan = download_hyperscan(None);

    Command::new("./bootstrap.sh")
        .current_dir(&boost)
        .status()
        .expect("Failed to execute process");
    Command::new("./b2")
        .current_dir(&boost)
        .status()
        .expect("Failed to execute process");

    let mut cmk = cmake::Config::new(hyperscan);
    cmk.env("-DBOOST_ROOT", boost.to_str().unwrap());
    cmk.build_target("hs");

    Ok(())
}

fn download_boost(version_: Option<&str>) -> PathBuf {
    // boost-1.81.0
    let version;
    if let Some(version_) = version_ {
        version = version_.to_string();
    } else {
        version = get_github_lastest_releases("boostorg", "boost").unwrap();
    }
    // 1.81.0
    let version_without_prefix = version.trim_start_matches("boost-");

    match download_url_tar_gz::<true, false>(
        "boost",
        format!("https://boostorg.jfrog.io/artifactory/main/release/{version_without_prefix}/source/{version}.tar.gz", version= version.replace("-", "_").replace(".", "_")).as_str(),
    ) {
        Ok(obj) => return obj,
        Err(e) => eprintln!("Error: {}", e),
    }

    if let None = version_ {
        match download_url_tar_gz::<false, false>(
            "boost",
            format!("https://github.com/boostorg/boost/release/lastest/{version}.tar.gz").as_str(),
        ) {
            Ok(obj) => return obj,
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    match download_github("boost", "boostorg", "boost", Some(version.as_ref())) {
        Ok(obj) => return obj,
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    unimplemented!("download_boost_git")
}

fn download_hyperscan(version: Option<&str>) -> PathBuf {
    match download_github_archive("hyperscan", "intel", "hyperscan", &version) {
        Ok(obj) => return obj,
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    unimplemented!("download_hyperscan_git")
}

fn download_github_archive(
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

    download_url_tar_gz::<false, true>(target_name, &url)
}

fn download_github(
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

        Repository::clone_recurse(&url, BASEDIR.join(target_name))?
            .checkout_head(Some(&mut checkout))?;

        Ok(BASEDIR.join(target_name))
    } else {
        let lastest = get_github_lastest_releases(user, repository)?;
        let url = format!("https://github.com/{user}/{repository}.git");
        let mut checkout = CheckoutBuilder::new();
        checkout.their_label(lastest.as_str());
        checkout.force();

        Repository::clone_recurse(&url, BASEDIR.join(target_name))?
            .checkout_head(Some(&mut checkout))?;

        Ok(BASEDIR.join(target_name))
    }
}

fn download_url_tar_gz<const CURL: bool, const STRIP: bool>(
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
    archive.unpack(BASEDIR.join(target_name))?;

    // 이름 변경
    if STRIP {
        std::fs::rename(
            std::fs::read_dir(BASEDIR.join(target_name))?
                .next()
                .unwrap()?
                .path(),
            BASEDIR.join("temp"),
        )?;
        std::fs::rename(BASEDIR.join("temp"), BASEDIR.join(target_name))?;
    }

    Ok(BASEDIR.join(target_name))
}

fn download_url_zip<const CURL: bool, const STRIP: bool>(
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
    archived.extract(BASEDIR.join(target_name))?;

    // 이름 변경
    if STRIP {
        std::fs::rename(
            std::fs::read_dir(BASEDIR.join(target_name))?
                .next()
                .unwrap()?
                .path(),
            BASEDIR.join("temp"),
        )?;
        std::fs::rename(BASEDIR.join("temp"), BASEDIR.join(target_name))?;
    }

    Ok(BASEDIR.join(target_name))
}

fn get_github_lastest_releases(
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
