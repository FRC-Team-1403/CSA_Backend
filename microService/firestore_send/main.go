package main

import (
	"encoding/json"
	"fmt"
	"os"
)

const debug = false

func main() {
	db := firebaseWrite{}
	var result map[string]interface{}
	jsonData := []byte(os.Args[1])
	err := json.Unmarshal(jsonData, &result)
	if err != nil {
		fmt.Println("Error parsing JSON:", err)
		return
	}
	title := fmt.Sprintf("%v", result["team"])
	db.Doc = title
	db.Collection = os.Args[2]
	db.WhatWrite = result
	err = db.One(db)
	if result["Points Lowest"] == 10000 {
		fmt.Println("data received was null")
		return
	}
	if err != nil {
		fmt.Println("failed sending to firestore", err)
		return
	}
	fmt.Println("success")
}
