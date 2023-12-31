package main

import (
	"bufio"
	"context"
	"fmt"
	"io"
	"log"
	"os"
	"os/signal"
	"strings"
	"syscall"
	"time"

	"github.com/libp2p/go-libp2p"
	"github.com/libp2p/go-libp2p/core/network"
	peerstore "github.com/libp2p/go-libp2p/core/peer"
	multiaddr "github.com/multiformats/go-multiaddr"
)

const protocolID = "/reverb/1.0.0"

func mustCopy(dst io.Writer, src io.Reader) {
	if _, err := io.Copy(dst, src); err != nil {
		log.Fatal(err)
	}
}

func echo(s network.Stream, shout string, delay time.Duration) {
	fmt.Fprintln(s, "\t", strings.ToUpper(shout))
	time.Sleep(delay)
	fmt.Fprintln(s, "\t", shout)
	time.Sleep(delay)
	fmt.Fprintln(s, "\t", strings.ToLower(shout))
}

func handler(s network.Stream) {
	input := bufio.NewScanner(s)
	for input.Scan() {
		go echo(s, input.Text(), 1*time.Second)
	}
	s.Close()
}

func main() {
	host, err := libp2p.New(
		libp2p.ListenAddrStrings("/ip4/127.0.0.1/tcp/0"),
	)
	if err != nil {
		panic(err)
	}
  defer host.Close()

	// print the host's PeerInfo in multiaddr format
	peerInfo := peerstore.AddrInfo{
		ID:    host.ID(),
		Addrs: host.Addrs(),
	}
	addrs, err := peerstore.AddrInfoToP2pAddrs(&peerInfo)
	log.Println("libp2p host address:", addrs[0])

	// configure handler
	host.SetStreamHandler(protocolID, handler)

	// if we received a peer address, we should connect to it
	if len(os.Args) > 1 {
		ma, err := multiaddr.NewMultiaddr(os.Args[1])
		if err != nil {
			panic(err)
		}
		peer, err := peerstore.AddrInfoFromP2pAddr(ma)
		if err != nil {
			panic(err)
		}
		if err := host.Connect(context.Background(), *peer); err != nil {
			panic(err)
		}
		s, err := host.NewStream(context.Background(), peer.ID, protocolID)
		if err != nil {
			panic(err)
		}

    go mustCopy(os.Stdout, s)
    mustCopy(s, os.Stdin)
	}

	ch := make(chan os.Signal)
	signal.Notify(ch, syscall.SIGKILL, syscall.SIGINT)
	<-ch
}
