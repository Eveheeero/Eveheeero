package main

import (
	"fmt"
	"jsonformatter/core"
	"os"
)

func main() {
	args := os.Args
	if len(args) < 2 {
		for {
			var path string
			fmt.Scanln(&path)
			format(path)
		}
	} else {
		target := args[1]
		format(target)
	}
}

func format(path string) {
	exist := fileExists(path)
	fmt.Println(exist)
	core.Test()
}

func fileExists(path string) bool {
	_, err := os.Stat(path)
	if err != nil {
		if os.IsNotExist(err) {
			return false
		}
	}
	return true
}
