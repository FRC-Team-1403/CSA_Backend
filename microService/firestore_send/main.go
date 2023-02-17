package main

import (
	"cloud.google.com/go/firestore"
	"encoding/json"
	"fmt"
	"log"
	"os"
)

func main() {
	if len(os.Args) == 0 || len(os.Args)%2 != 0 {
		log.Fatalln("Incorrect Args")
	}
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
	for x := 2; x < len(os.Args); x++ {
		builder = app.Client.Collection(os.Args[x]).Doc(os.Args[x+1])
	}
	_, err = builder.Set(app.Ctx, result, firestore.MergeAll)
	if err != nil {
		fmt.Println("Error While Sending ", err)
		return
	}
	fmt.Println("Success")
}
