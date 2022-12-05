package main

import (
	"os"
	"os/exec"
	"runtime"
)

func main() {
	tempdir, _ := os.MkdirTemp("", "hyperscan")
	defer os.RemoveAll(tempdir)

	// Download hyperscan
	download(tempdir, "hyperscan", "https://github.com/intel/hyperscan/archive/refs/tags/v5.4.0.tar.gz", "https://github.com/intel/hyperscan", "v5.4.0")
	download(tempdir, "boost", "https://boostorg.jfrog.io/artifactory/main/release/1.80.0/source/boost_1_80_0.tar.gz", "https://github.com/boostorg/boost", "boost-1.80.0")

	// Boost build
	if runtime.GOOS == "windows" {
		cmd := exec.Command(tempdir+"/boost/bootstrap.bat", "cxxflags=-fPIC", "cflags=-fPIC")
		cmd.Dir = tempdir + "/boost"
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
		cmd.Run()

		cmd = exec.Command(tempdir+"/boost/b2.exe", "--build-type=complete", "--layout=tagged", "runtime-link=static", "runtime-link=static", "variant=release", "threading=multi")
		cmd.Dir = tempdir + "/boost"
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
		cmd.Run()
	} else {
		cmd := exec.Command(tempdir+"/boost/bootstrap.sh", "cxxflags=-fPIC", "cflags=-fPIC")
		cmd.Dir = tempdir + "/boost"
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
		cmd.Run()

		cmd = exec.Command(tempdir+"/boost/b2", "--build-type=complete", "--layout=tagged", "runtime-link=static", "runtime-link=static", "variant=release", "threading=multi")
		cmd.Dir = tempdir + "/boost"
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
		cmd.Run()
	}

	// Hyperscan build
	os.Mkdir(tempdir+"/hyperscan/build", 0777)
	cmd := exec.Command("cmake", "..", "-DBOOST_ROOT="+tempdir+"/boost", "-DBUILD_STATIC_AND_SHARED=ON", "-DCMAKE_BUILD_TYPE=Release", "-DCMAKE_C_FLAGS=\"-fPIC\"", "-DCMAKE_CXX_FLAGS=\"-fPIC\"", "-DFAT_RUNTIME=off", "-GNinja")
	cmd.Dir = tempdir + "/hyperscan/build"
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Run()

	cmd = exec.Command("ninja", "hs")
	cmd.Dir = tempdir + "/hyperscan/build"
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Run()

	// Copy files
	os.Rename(tempdir+"/hyperscan/build/lib/libhs.a", "libhs.a")
	os.Rename(tempdir+"/hyperscan/build/lib/libhs.lib", "libhs.lib")
	// os.Rename(tempdir+"/hyperscan/build/lib/libhs.dll", "libhs.dll")
	// os.Rename(tempdir+"/hyperscan/build/lib/libhs.so", "libhs.so")
	// entries, _ := os.ReadDir(tempdir + "/hyperscan/build/lib")
	// for _, entry := range entries {
	// 	if entry.Type().IsRegular() && !entry.IsDir() && entry.Name()[0:8] == "libhs.so" {
	// 		os.Rename(tempdir+"/hyperscan/build/lib/"+entry.Name(), entry.Name())
	// 	}
	// }

	println("Done")
}

func download(tempdir string, name string, url string, git string, tag string) {
	cmd := exec.Command("curl", "-o", tempdir+"/"+name+".tar.gz", "-L", url)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err == nil {
		_, err = os.Stat(tempdir + "/" + name + ".tar.gz")
	}
	if err != nil {
		download_via_git(tempdir, name, git, tag)
		return
	}

	cmd = exec.Command("tar", "-xvf", tempdir+"/"+name+".tar.gz", "-C", tempdir)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err = cmd.Run()
	if err != nil {
		download_via_git(tempdir, name, git, tag)
		return
	}
	if tag[0] == 'v' {
		tag = tag[1:]
	}
	os.Rename(tempdir+"/"+name+"-"+tag, tempdir+"/"+name)
}

func download_via_git(tempdir string, name string, git string, tag string) {
	cmd := exec.Command("git", "clone", git, "-b", tag, tempdir+"/"+name, "--recursive")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		panic(err)
	}
}
