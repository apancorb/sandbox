package main

import (
	"fmt"
	"log"
	"net/http"
	"os"

	"golang.org/x/net/html"
)

// extracts a list of links from a given url
func extract(url string) ([]string, error) {
	resp, err := http.Get(url)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("getting %s: %s", url, resp.Status)
	}

	doc, err := html.Parse(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("parsing %s as HTML: %v", url, err)
	}

	var links []string
	forEachNode(doc, func(n *html.Node) {
		if n.Type == html.ElementNode && n.Data == "a" {
			for _, a := range n.Attr {
				if a.Key != "href" {
					continue
				}
				link, err := resp.Request.URL.Parse(a.Val)
				if err != nil {
					continue // ignore bad URLs
				}
				links = append(links, link.String())
			}
		}
	})
	return links, nil
}

func forEachNode(n *html.Node, visitNode func(n *html.Node)) {
	if visitNode != nil {
		visitNode(n)
	}
	for c := n.FirstChild; c != nil; c = c.NextSibling {
		forEachNode(c, visitNode)
	}
}

func crawl(url string) []string {
	fmt.Println(url)
	list, err := extract(url)
	if err != nil {
		log.Print(err)
	}
	return list
}

// starting from the command-line arguments,
// crawl the web breadth-first
func main() {
  args := os.Args[1:]
  if args == nil || len(args) < 1 {
    log.Fatalln("Initial list of web urls missing")
  }

  // worklist represnts a queue of
  // all links found by workers
  worklist := make(chan []string)
  // unseen links for workers to work on
  unseenLinks := make(chan string)

  // create 20 workers
  for i := 0; i < 20; i++ {
    go func() {
      for link := range unseenLinks {
        foundLinks := crawl(link)
        go func() { worklist <- foundLinks }()
      }
    }()
  }

  // provide initial list of links 
  // to avoid deadlock in main goroutine
  go func() { worklist <- args }()

  seen := make(map[string]bool)
  for list := range worklist {
    for _, link := range list {
      if !seen[link] {
        seen[link] = true
        unseenLinks <- link
      }
    }
  }
}
