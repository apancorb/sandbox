package main

import (
	"context"
	"log"
	"os"
	"os/signal"
	"syscall"

	pb "pow/pb.pow"

	"github.com/libp2p/go-libp2p"
	pubsub "github.com/libp2p/go-libp2p-pubsub"
	"github.com/libp2p/go-libp2p/core/peer"
	"google.golang.org/protobuf/proto"
)

// publish forwards the miner's view of its blockchain
// to all peers in the network
func publish(ctx context.Context, pub *pubsub.Topic, m *Miner) {
	for {
		// receive new mined block
		blockchain := <-m.Result
		// serialize blockchain
		sendBlockchain := pb.Blockchain{}
		for _, block := range blockchain {
			sendBlockchain.Blockchain = append(sendBlockchain.Blockchain, &pb.Block{
				PrevBlockHash: block.PrevBlockHash,
				Data:          block.Data,
				Hash:          block.Hash,
				Nonce:         block.Nonce})
		}
		out, err := proto.Marshal(&sendBlockchain)
		if err != nil {
			panic(err)
		}
		// propagate block in the p2p network
		pub.Publish(ctx, out)
	}
}

// subscribe listens to new blockchains being advertized
// by other peers in the network
func subscribe(ctx context.Context, sub *pubsub.Subscription, hostID peer.ID, m *Miner) {
	for {
		// receive new block from a peer
		msg, err := sub.Next(ctx)
		if err != nil {
			panic(err)
		} else if msg.ReceivedFrom == hostID {
			// only consider messages delivered
			// by other peers
			continue
		}
		log.Println("Received Blockchain from:", msg.ReceivedFrom.Pretty())

		// deserialize blockchain
		recvBlockchain := &pb.Blockchain{}
		if err := proto.Unmarshal(msg.Data, recvBlockchain); err != nil {
			panic(err)
		}
		blockchain := Blockchain{}
		for _, block := range recvBlockchain.Blockchain {
			blockchain = append(blockchain, Block{
				PrevBlockHash: block.PrevBlockHash,
				Data:          block.Data,
				Hash:          block.Hash,
				Nonce:         block.Nonce})
		}

		// validate and proccess newly discoverd block
		m.Discover <- blockchain
	}
}

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
	ctx, cancel := context.WithCancel(context.Background())
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
	go miner.Start(ctx)
	defer miner.Shutdown()

	go publish(ctx, pub, miner)
	go subscribe(ctx, sub, host.ID(), miner)

	ch := make(chan os.Signal)
	signal.Notify(ch, syscall.SIGKILL, syscall.SIGINT)
	<-ch
	cancel()
}
