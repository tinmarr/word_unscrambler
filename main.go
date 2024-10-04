package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
	"strings"
)

func letterIndex(letter rune) int {
	return int(letter - 'a')
}

func loadDictionary() map[string][]string {
	data, err := os.ReadFile("./assets/DL.txt")
	if err != nil {
		fmt.Println("Unable to read dictionary file")
		os.Exit(1)
	}
	dict := make(map[string][]string)
	for _, v := range strings.Split(string(data), "\n") {
		index := word2Int(v).String()
		if dict[index] == nil {
			dict[index] = []string{v}
		} else {
			dict[index] = append(dict[index], v)
		}
	}
	return dict
}

func word2Int(word string) *big.Int {
	wordInt := big.NewInt(0)
	for _, v := range word {
		ind := big.NewInt(int64(4 * letterIndex(v)))
		ind.Exp(big.NewInt(2), ind, nil)
		wordInt.Add(wordInt, ind)
	}
	return wordInt
}

func main() {
	dict := loadDictionary()
	reader := bufio.NewScanner(os.Stdin)
	for {
		fmt.Print("Enter a scrambled word: ")
		if !reader.Scan() {
			break
		}
		word := strings.ToLower(reader.Text())
		ind := word2Int(word)

		fmt.Println(dict[ind.String()])
	}
}
