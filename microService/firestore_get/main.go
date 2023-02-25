package main

import (
	"context"
	"errors"
	"fmt"
	"log"
	"os"

	"cloud.google.com/go/firestore"
	firebase "firebase.google.com/go"
	"google.golang.org/api/option"
)

type Firestore struct {
	Client *firestore.Client
	Ctx    context.Context
	App    *firebase.App
}

func (Firestore) init() (s Firestore, err error) {
	s.Ctx = context.Background()
	var file option.ClientOption = option.WithCredentialsFile("microService/admin.json")
	s.App, err = firebase.NewApp(s.Ctx, nil, file)
	if err != nil {
		return s, errors.New("Failed due to: " + err.Error())
	}
	s.Client, err = s.App.Firestore(s.Ctx)
	if err != nil {
		return s, errors.New("Failed due to: " + err.Error())
	}
	return s, err
}

func (Firestore) close(r Firestore) (err error) {
	err = r.Client.Close()
	if err != nil {
		return errors.New("Failed due to: " + err.Error())
	}
	return err
}

func main() {
	if len(os.Args) < 1 {
		log.Panic("Please Give OS Args")
	}
	db, err := Firestore{}.init()
	if err != nil {
		log.Panic("Failed To Start Firestore Client due to:", err)
	}
	defer db.close(db)
	data, err := db.Client.Collection(os.Args[1]).Doc(os.Args[2]).Collection(os.Args[3]).Doc(os.Args[4]).Get(db.Ctx)
	if err != nil {
		log.Panic("Failed to Get data due to: ", err)
	}
	fmt.Print(data)
}
