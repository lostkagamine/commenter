package main

import (
	"fmt"
	"log"
	"net/http"
	"time"

	"bytes"
	"encoding/json"

	"github.com/gobuffalo/pop"
	"github.com/labstack/echo/v4"
	"kagamine.tech/commenter/models"
)

func must(e error) {
	if e != nil {
		log.Panic(e)
	}
}

type submitpayload struct {
	Author string
	Text   string
}

func main() {
	e := echo.New()

	tx, err := pop.Connect("development")
	must(err)

	e.POST("/submit", func(c echo.Context) error {
		body := c.Request().Body
		buf := new(bytes.Buffer)
		buf.ReadFrom(body)
		bodyBytes := buf.Bytes()
		var bodyObj submitpayload
		must(json.Unmarshal(bodyBytes, &bodyObj))
		dbEntry := models.Comment{
			CreatedAt: time.Now(),
			Author:    bodyObj.Author,
			Text:      bodyObj.Text,
		}
		_, dbErr := tx.ValidateAndSave(&dbEntry)
		must(dbErr)

		return c.NoContent(http.StatusOK)
	})

	e.GET("/", func(c echo.Context) error {
		res := ""

		comments := []models.Comment{}
		err := tx.All(&comments)
		must(err)

		res = res + fmt.Sprintf("There are <strong>%d</strong> comments.<br>", len(comments))

		for _, i := range comments {
			res = res + fmt.Sprintf("<strong>%s</strong>: %s<br>", i.Author, i.Text)
		}

		return c.HTML(http.StatusOK, res)
	})

	e.Logger.Fatal(e.Start(":3000"))
}
