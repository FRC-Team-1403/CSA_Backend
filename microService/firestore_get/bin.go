package main

import (
	"fmt"
	"log"
	"os"
)

func main() {
	if len(os.Args) < 1 {
		log.Panic("Please Give OS Args")
	}
	fire := Firestore{}
	db, err := fire.Init()
	if err != nil {
		log.Panic("Failed To Start Firestore Client due to:", err)
	}
	defer db.Close(db)
	data, err := db.Client.Collection(os.Args[1]).Doc(os.Args[2]).Get(db.Ctx)
	if err != nil {
		log.Panic("Failed to Get data due to: ", err)
	}
	fmt.Print(data)
}
