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
		if os.Args[1] == "get" {
			send := firebaseRead{}
			send.Path = os.Args[2]
			send.Id = os.Args[3]
			jsonReturn := map[string]interface{}{
				"Average Auto Contributed":   0.0,
				"Average Points Contributed": 0.0,
				"Average Telop Contributed":  0.0,
				"Error":                      "Good",
			}
			defer fmt.Println(jsonReturn)
			err, data := send.One(send)
			if err != nil {
				jsonReturn["Error"] = err
				return
			}
			jsonReturn["Average Auto Contributed"] = data.OneData["Average Auto Contributed"]
			jsonReturn["Average Points Contributed"] = data.OneData["Average Points Contributed"]
			jsonReturn["Average Telop Contributed"] = data.OneData["Average Telop Contributed"]
			return
		}
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
