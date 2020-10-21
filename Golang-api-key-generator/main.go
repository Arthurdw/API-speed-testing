package main

import (
	"encoding/json"
	"fmt"
	"math/rand"
	"net/http"
	"strconv"
)

const (
	Base = "/api/generator/"
	Charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~."
	Port = "8000"
)

func main() {
	http.HandleFunc(Base + "key", handler)
	fmt.Printf("Server starting on localhost:%v\n", Port)
	panic(http.ListenAndServe(":" + Port, nil))
}

type Response struct {
	Key string `json:"key"`
}

func handler(w http.ResponseWriter, r *http.Request) {
	length, ok := r.URL.Query()["length"]
	if !ok {
		panic("invalid query 'length'")
	}

	intLength, err := strconv.Atoi(length[0])
	if err != nil {
		panic(err)
	}

	var result string
	for i := 0; i < intLength; i++ {
		result += string(Charset[getRandomNr(0, len(Charset) - 1)])
	}

	bytes, _ := json.Marshal(Response{result})

	w.Header().Set("Content-Type", "application/json")
	_, err = w.Write(bytes)
	if err != nil {
		panic(err)
	}
}

func getRandomNr(min int, max int) int {
	return rand.Intn(max - min + 1) + min
}