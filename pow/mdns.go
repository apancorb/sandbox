package main

import (
	"context"
	"log"

	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/libp2p/go-libp2p/p2p/discovery/mdns"
)

// discoveryNotifee gets notified when we find
// a new peer via mDNS discovery
type discoveryNotifee struct {
	host host.Host
}

// HandlePeerFound connects to peers discovered via mDNS. Once they're connected,
// the PubSub system will automatically start interacting with them if they also
// support PubSub.
func (n *discoveryNotifee) HandlePeerFound(pi peer.AddrInfo) {
	log.Println("Discovered new peer:", pi.ID.Pretty())
	err := n.host.Connect(context.Background(), pi)
	if err != nil {
		log.Printf("Error connecting to peer %s: %s\n", pi.ID.Pretty(), err)
	}
	// send blockchain data
}

// setupDiscovery creates an mDNS discovery service and attaches it to the libp2p Host.
// This lets us automatically discover peers on the same LAN and connect to them.
func setupDiscovery(h host.Host) error {
	s := mdns.NewMdnsService(h, "pow", &discoveryNotifee{host: h})
	return s.Start()
}
