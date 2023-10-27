package main

import (
	"bytes"
	"context"
	"crypto/rand"
	"crypto/sha256"
	"encoding/base64"
	"errors"
	"fmt"
	"log"
	"math"
	"math/big"
	"strconv"
)

const (
	TargetBits = 20 // Adjust to control the difficulty of the PoW algorithm
	MaxNonce   = math.MaxInt64
)

type Block struct {
	PrevBlockHash []byte
	Data          []byte
	Hash          []byte
	Nonce         int64
}

type Blockchain []Block

type Miner struct {
	// the miner's blockchain
	Blockchain Blockchain

	// Pow algorithm difficulty
	Target *big.Int

	// once the host receives a new block
	// from a host in the p2p network it will
	// be sent to this channel such that the
	// miner can validate and process the new block
	Discover chan Blockchain

	// once the miner mines a new valid block,
	// it will send it to the host in this channel
	// so that other hosts in the p2p network
	// can be notified
	Result chan Blockchain
}

func New() *Miner {
	target := big.NewInt(1)
	target.Lsh(target, uint(256-TargetBits))

	return &Miner{
		Blockchain: make(Blockchain, 0),
		Target:     target,
		Discover:   make(chan Blockchain),
		Result:     make(chan Blockchain)}
}

func (m *Miner) Start(ctx context.Context) {
loop:
	for {
		mctx, mcancel := context.WithCancel(ctx)
		out := make(chan Block)
		go m.Mine(mctx, out)

		select {
		case <-ctx.Done():
			mcancel()
			break loop
		case blockchain := <-m.Discover:
			err := m.Validate(blockchain)
			if err != nil {
				log.Println("Invalid Blockchain:", err.Error())
			} else {
				m.Blockchain = blockchain 
				log.Println("New Blockchain Discovered:", m.Blockchain)
				mcancel()
			}
		case block := <-out:
			m.Blockchain = append(m.Blockchain, block)
			log.Println("New Block Mined:", m.Blockchain)
			m.Result <- m.Blockchain
		}
	}
}

func (m *Miner) Shutdown() {
	close(m.Discover)
	close(m.Result)
}

func (m *Miner) Validate(otherBlockchain Blockchain) error {
	// skip proposed blockchain if it is smaller
	// than ours
	if len(m.Blockchain) > len(otherBlockchain) {
		return errors.New("Smaller Blockchain")
	}

	for i := 1; i < len(otherBlockchain); i++ {
		currBlock := otherBlockchain[i]
		prevBlock := otherBlockchain[i-1]

		// check if the previous block hash points
		// to the current block in the chain
		if !bytes.Equal(currBlock.PrevBlockHash, prevBlock.Hash[:]) {
			return errors.New("PrevBlockHash Mismatch")
		}

		// check if the hash (solution to the PoW algorithm)
		// matches with the determined difficulty level
		var hashInt big.Int
		data := prepareData(currBlock)
		hash := sha256.Sum256(data)
		hashInt.SetBytes(hash[:])

		if hashInt.Cmp(m.Target) != -1 {
			return errors.New("Target Mismatch")
		}
	}

	return nil
}

func (m *Miner) Mine(ctx context.Context, out chan<- Block) error {
	var hashInt big.Int
	var hash [32]byte
	var nonce int64
	var prevBlockHash []byte
	if len(m.Blockchain) > 0 {
		prevBlockHash = m.Blockchain[len(m.Blockchain)-1].Hash
	}
	data := randomString(10)

	fmt.Printf("Mining a new block with difficulty %d\n", TargetBits)

	for nonce < MaxNonce {
		block := Block{
			PrevBlockHash: prevBlockHash,
			Data:          []byte(data),
			Nonce:         nonce}

		hash = sha256.Sum256(prepareData(block))
		hashInt.SetBytes(hash[:])
		fmt.Printf("\r%x", hash)

		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			if hashInt.Cmp(m.Target) == -1 {
				block.Hash = hash[:]
				out <- block
				break
			} else {
				nonce++
			}
		}
	}

	return ctx.Err()
}

func prepareData(block Block) []byte {
	data := bytes.Join(
		[][]byte{
			block.PrevBlockHash,
			block.Data,
			[]byte(strconv.FormatInt(int64(block.Nonce), 10)),
			[]byte(strconv.FormatInt(int64(TargetBits), 10)),
		},
		[]byte{},
	)
	return data
}

func randomString(length int) string {
	buffer := make([]byte, length)
	rand.Read(buffer)
	return base64.URLEncoding.EncodeToString(buffer)[:length]
}
