package main

import (
	"cloud.google.com/go/firestore"
	"encoding/json"
	"fmt"
	"os"
)

func main() {
	var result map[string]interface{}
	jsonData := []byte(os.Args[1])
	err := json.Unmarshal(jsonData, &result)
	if err != nil {
		fmt.Println("Error parsing JSON:", err)
		return
	}
	title := fmt.Sprintf("%v", result["team"])
	//for nested send
	if os.Args[3] != "" {
		set_match(title, result)
	}
	set_year(title, result)
}

func set_year(title string, result map[string]interface{}) {
	db := firebaseWrite{}
	db.Doc = title
	db.Collection = os.Args[2]
	db.WhatWrite = result
	err := db.One(db)
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

func set_match(title string, result map[string]interface{}) {
	client := Firestore{}
	app, err := client.Init()
	if err != nil {
		fmt.Println("failed to start firestore")
		return
	}
	_, err = app.Client.Collection(os.Args[2]).Doc(title).Collection("Matches").Doc(os.Args[3]).Set(app.Ctx, result, firestore.MergeAll)
	if err != nil {
		fmt.Println("Failed to send because: ", err)
		return
	}
	fmt.Println("success")
	return
}
