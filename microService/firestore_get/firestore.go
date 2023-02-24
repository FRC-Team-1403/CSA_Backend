package main

import (
	"cloud.google.com/go/firestore"
	"context"
	"errors"
	firebase "firebase.google.com/go"
	"google.golang.org/api/option"
)

type Firestore struct {
	Client *firestore.Client
	Ctx    context.Context
	App    *firebase.App
}

func (Firestore) Init() (s Firestore, err error) {
	s.Ctx = context.Background()
	var file option.ClientOption
	file = option.WithCredentialsFile("microService/admin.json")
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

func (Firestore) Close(r Firestore) (err error) {
	err = r.Client.Close()
	if err != nil {
		return errors.New("Failed due to: " + err.Error())
	}
	return err
}
