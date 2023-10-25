package main

import (
	"bytes"
	"context"
	"encoding/binary"
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/libp2p/go-libp2p"
	pubsub "github.com/libp2p/go-libp2p-pubsub"
	"github.com/libp2p/go-libp2p/core/peer"
)

func main() {
	// create a new libp2p Host that listens on a random TCP port
	host, err := libp2p.New(libp2p.ListenAddrStrings("/ip4/0.0.0.0/tcp/0"))
	if err != nil {
		panic(err)
	}
	defer host.Close()
	log.Println("host ID:", host.ID().Pretty())

	// setup local mDNS discovery
	if err := setupDiscovery(host); err != nil {
		panic(err)
	}

	// create a new PubSub service using the GossipSub router
	ctx := context.Background()
	gossipSub, err := pubsub.NewGossipSub(ctx, host)
	if err != nil {
		panic(err)
	}

	// join pubsub topic
	pub, err := gossipSub.Join("pow")
	if err != nil {
		panic(err)
	}
	defer pub.Close()

	// subscribe to topic
	sub, err := pub.Subscribe()
	if err != nil {
		panic(err)
	}
	defer sub.Cancel()

	// create PoW miner
	miner := New()
	go miner.Start()
	defer miner.Shutdown()

	go publish(ctx, pub, miner)
	go subscribe(ctx, sub, host.ID(), miner)

	ch := make(chan os.Signal)
	signal.Notify(ch, syscall.SIGKILL, syscall.SIGINT)
	<-ch
}

// start publisher to topic
func publish(ctx context.Context, pub *pubsub.Topic, m *Miner) {
	for {
		// receive new mined block
		block := <-m.Result
    fmt.Println("here", block)
		// serialize block
		buf := &bytes.Buffer{}
		if err := binary.Write(buf, binary.BigEndian, block); err != nil {
			panic(err)
		}
		// propagate block in the p2p network
		pub.Publish(ctx, buf.Bytes())
	}
}

// start subsriber to topic
func subscribe(ctx context.Context, sub *pubsub.Subscription, hostID peer.ID, m *Miner) {
	for {
		// receive new block from a peer
		msg, err := sub.Next(ctx)
		if err != nil {
			panic(err)
		}

		// only consider messages delivered
		// by other peers
		if msg.ReceivedFrom == hostID {
			continue
		}

		// deserialize block
		block := Block{}
		buf := &bytes.Buffer{}
		if _, err = buf.Write(msg.Data); err != nil {
			panic(err)
		}
		if err = binary.Read(buf, binary.BigEndian, &block); err != nil {
			panic(err)
		}

		log.Printf("Received Block: %s, from: %s\n", string(msg.Data), msg.ReceivedFrom.Pretty())

		m.Discover <- block
	}
}
