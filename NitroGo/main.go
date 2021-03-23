package main

import (
	"fmt"
	"os"
	"os/signal"
	"strings"
	"time"

	"github.com/Nv7-Github/chromego"
)

func handle(err error) {
	if err != nil {
		panic(err)
	}
}

func main() {
	drv, err := chromego.CreateDriver("chromedriver", 3000)
	handle(err)

	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	go func() {
		<-c
		fmt.Println("Gracefully shutting down...")
		err = drv.Close()
		handle(err)
		os.Exit(0)
	}()

	err = drv.Navigate("https://nitrotype.com/race")
	handle(err)

	time.Sleep(5 * time.Second)

	startBtn, err := drv.FindByXPath(`//*[@id="raceContainer"]/div[3]/div[1]/div[1]/div[3]/div/button`)
	handle(err)
	err = startBtn.Click()
	handle(err)

	time.Sleep(5 * time.Second)

	dash, err := drv.FindByCSSSelector(".dash-copy")
	handle(err)

	txt, err := dash.Text()
	handle(err)

	body, err := drv.FindByCSSSelector("body")
	handle(err)

	err = body.Type(txt)
	handle(err)

	time.Sleep(time.Second * 5)

	for {
		err = drv.Refresh()
		handle(err)

		time.Sleep(time.Second * 5)

		for {
			time.Sleep(time.Second / 2)
			msg, err := drv.FindByCSSSelector(".nmDash-message")
			handle(err)
			txt, err := msg.Text()
			if err != nil || txt != "Please wait for the race to begin!" {
				break
			}
		}

		time.Sleep(5 * time.Second)

		for {
			dash, err = drv.FindByCSSSelector(".dash-copy")
			handle(err)

			txt, err = dash.Text()
			if err != nil {
				break
			}

			body, err = drv.FindByCSSSelector("body")
			handle(err)

			txt = strings.Replace(txt, "\n", " ", -1)

			err = body.Type(txt)
			handle(err)

			time.Sleep(time.Second / 2)

			scoreElem, err := drv.FindByXPath(`//*[@id="raceContainer"]/div[1]/div[2]/div[2]/div/div[1]/div/div[1]/div[4]/div[2]/div[2]/div/div[4]`)
			handle(err)
			_, err = scoreElem.Text()
			if err == nil {
				break
			}
		}

		time.Sleep(time.Second * 5)

		scoreElem, err := drv.FindByXPath(`//*[@id="raceContainer"]/div[1]/div[2]/div[2]/div/div[1]/div/div[1]/div[4]/div[2]/div[2]/div/div[4]`)
		handle(err)
		score, err := scoreElem.Text()
		handle(err)
		fmt.Println(score)
	}
}
