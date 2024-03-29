package main

import (
	"cloud.google.com/go/firestore"
	"context"
	"errors"
	firebase "firebase.google.com/go"
	"google.golang.org/api/option"
)

type firebaseWrite struct {
	Collection string
	Doc        string
	WhatWrite  map[string]interface{}
}

func (firebaseWrite) One(r firebaseWrite) (err error) {
	app := Firestore{}
	app, err = app.Init()
	if err != nil {
		return errors.New("Failed due to: " + err.Error())
	}
	_, err = app.Client.Collection(r.Collection).Doc(r.Doc).Set(app.Ctx, r.WhatWrite)
	if err != nil {
		return errors.New("Failed due to: " + err.Error())
	}
	err = app.Close(app)
	if err != nil {
		return errors.New("Failed due to: " + err.Error())
	}

	return err
}

type Firestore struct {
	Client *firestore.Client
	Ctx    context.Context
	App    *firebase.App
	Write  firebaseWrite
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

type firebaseRead struct {
	Path     string
	Id       string
	ManyData []map[string]interface{}
	OneData  map[string]interface{}
}

func (firebaseRead) One(r firebaseRead) (err error, s firebaseRead) {
	app := Firestore{}
	app, err = Firestore{}.Init()
	if err != nil {
		return errors.New("Failed due to: " + err.Error()), r
	}
	dsnap, err := app.Client.Collection(r.Path).Doc(r.Id).Get(app.Ctx)
	if err != nil {
		return errors.New("Failed due to: " + err.Error()), r
	}
	err = app.Close(app)
	if err != nil {
		return errors.New("Failed due to: " + err.Error()), r
	}
	r.OneData = dsnap.Data()
	return err, r
}
