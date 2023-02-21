pub mod utils;

use std::{path::PathBuf, process::Command};
use utils::*;

pub fn build(basedir: impl Into<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let basedir: PathBuf = basedir.into();

    let boost = download_boost(&basedir, Some("boost-1.81.0"));
    let hyperscan = download_hyperscan(&basedir, None);

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

pub fn download_boost(basedir: &PathBuf, version_: Option<&str>) -> PathBuf {
    // boost-1.81.0
    let version;
    if let Some(version_) = version_ {
        version = version_.to_string();
    } else {
        version = get_github_lastest_releases("boostorg", "boost").unwrap();
    }
    // 1.81.0
    let version_without_prefix = version.trim_start_matches("boost-");

    match download_url_tar_gz::<true, false>(basedir,
        "boost",
        format!("https://boostorg.jfrog.io/artifactory/main/release/{version_without_prefix}/source/{version}.tar.gz", version= version.replace("-", "_").replace(".", "_")).as_str(),
    ) {
        Ok(obj) => return obj,
        Err(e) => eprintln!("Error: {}", e),
    }

    if let None = version_ {
        match download_url_tar_gz::<true, false>(
            basedir,
            "boost",
            format!("https://github.com/boostorg/boost/release/lastest/{version}.tar.gz").as_str(),
        ) {
            Ok(obj) => return obj,
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    match download_github(
        basedir,
        "boost",
        "boostorg",
        "boost",
        Some(version.as_ref()),
    ) {
        Ok(obj) => return obj,
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    unimplemented!("download_boost_git")
}

pub fn download_hyperscan(basedir: &PathBuf, version: Option<&str>) -> PathBuf {
    match download_github_archive(basedir, "hyperscan", "intel", "hyperscan", &version) {
        Ok(obj) => return obj,
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    unimplemented!("download_hyperscan_git")
}
