package main

import (
	"fmt"
	"os"
	"os/signal"
	"time"
)

type empty struct{}

func initHTML() {
	// Play
	btn, err := drv.FindByCSSSelector(".pz-modal__button")
	handle(err)
	err = btn.Click()
	handle(err)
	time.Sleep(time.Second / 10) // Wait for play dialog to fade

	// Init button map
	btns, err := drv.FindElemsByCSSSelector(".hive-cell")
	handle(err)
	for i, val := range btns {
		text, err := val.Text()
		handle(err)
		letters[text] = empty{}
		if i == 0 {
			center = text
		}

		btnMap[text] = val
	}
	ent, err := drv.FindByCSSSelector(".hive-action__submit")
	handle(err)
	btnMap["ent"] = ent

	// Get Other Stuff
	textBox, err = drv.FindByCSSSelector(".sb-hive-input")
	handle(err)
}

func setup() {
	initHTML()

	loadWords()
}

func wait() {
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	_ = <-c
	fmt.Println("Gracefully shutting down...")
	cleanup()
}

func cleanup() {
	err := drv.Close()
	if err != nil {
		panic(err)
	}
}
