package main

import (
    "fmt"
    "bufio"
    "os"
    "strings"

    "github.com/jdkato/prose"
)

func main() {
    // Create a new document with the default configuration:
    reader := bufio.NewReader(os.Stdin)
    fmt.Print("Enter sentence: ")
    sent, _ := reader.ReadString('\n')
    doc, _ := prose.NewDocument(sent)

    done := []string{}

    // Iterate over the doc's tokens:
    for _, tok := range doc.Tokens() {
      if tok.Tag == "JJ" {
        if !(isIn(tok.Tag, done)) {
          done = append(done, tok.Tag)
          sent = strings.Replace(sent, tok.Text, "hella-" + tok.Text, -1)
        }
      }
    }


    fmt.Println(sent)
}

func isIn(a string, list []string) bool {
    for _, b := range list {
        if b == a {
            return true
        }
    }
    return false
}
