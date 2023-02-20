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
	client := Firestore{}
	app, err := client.Init()
	if err != nil {
		fmt.Println("failed to start firestore")
		return
	}
	var builder *firestore.DocumentRef
	if len(os.Args) < 5 {
		builder = app.Client.Collection(os.Args[2]).Doc(os.Args[3])
	} else {
		builder = app.Client.Collection(os.Args[2]).Doc(os.Args[3]).Collection(os.Args[4]).Doc(os.Args[5])
	}
	_, err = builder.Set(app.Ctx, result, firestore.MergeAll)
	if err != nil {
		fmt.Println("Error While Sending ", err)
		return
	}
	fmt.Println("Success")
}
