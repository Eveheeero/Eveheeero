pub mod utils;

use std::{path::PathBuf, process::Command};
use utils::*;

pub fn build(basedir: impl Into<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let basedir: PathBuf = basedir.into();
    std::fs::create_dir_all(&basedir)?;
    let basedir = basedir.canonicalize()?;
    let mut boost = basedir.join("boost");
    let mut hyperscan = basedir.join("hyperscan");

    #[cfg(windows)]
    let exec_ext = ".exe";
    #[cfg(not(windows))]
    let exec_ext = "";
    #[cfg(windows)]
    let script_ext = ".bat";
    #[cfg(not(windows))]
    let script_ext = ".sh";

    if !basedir.join("boost").exists() {
        boost = download_boost(&basedir, None);
    }
    if !basedir.join("hyperscan").exists() {
        hyperscan = download_hyperscan(&basedir, None);
    }

    if !boost.join("b2").exists() {
        Command::new(format!("./bootstrap{script_ext}"))
            .args(["cflags=-fPIC", "cxxflags=-fPIC"])
            .current_dir(&boost)
            .status()
            .expect("Boost bootstrap failed");
    }

    if !boost.join("bin.v2").exists() {
        Command::new(format!("./b2{exec_ext}"))
            .args([
                "--build-type=complete",
                "--layout=tagged",
                "runtime-link=static",
                "runtime-link=static",
                "variant=release",
                "threading=multi",
            ])
            .current_dir(&boost)
            .status()
            .expect("Boost build failed");
    }

    if !hyperscan.join("build").exists() {
        std::fs::create_dir(hyperscan.join("build"))?;

        Command::new("cmake")
            .args([
                "..",
                format!("-DBOOST_ROOT={}", boost.to_str().unwrap()).as_str(),
                "-DBUILD_STATIC_AND_SHARED=ON",
                "-DCMAKE_BUILD_TYPE=Release",
                "-DCMAKE_C_FLAGS=\"-fPIC\"",
                "-DCMAKE_CXX_FLAGS=\"-fPIC\"",
                "-DFAT_RUNTIME=on",
            ])
            .current_dir(&hyperscan.join("build"))
            .status()
            .expect("CMake generation failed");

        Command::new("cmake")
            .args(["--build", "."])
            .current_dir(&hyperscan.join("build"))
            .status()
            .expect("CMake build failed");
    }

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

    match download_url_tar_gz::<true, true>(basedir,
        "boost",
        format!("https://boostorg.jfrog.io/artifactory/main/release/{version_without_prefix}/source/{version}.tar.gz", version= version.replace("-", "_").replace(".", "_")).as_str(),
    ) {
        Ok(obj) => return obj,
        Err(e) => eprintln!("Error: {}", e),
    }

    if let None = version_ {
        match download_url_tar_gz::<false, true>(
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
