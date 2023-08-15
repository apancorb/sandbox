// Implements the SHA256 hash algorithm as defined
// in FIPS 180-4.
//
// https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
package main

import (
	"encoding/binary"
	"fmt"
	"os"
)

const (
	chunk = 64
	init0 = 0x6A09E667
	init1 = 0xBB67AE85
	init2 = 0x3C6EF372
	init3 = 0xA54FF53A
	init4 = 0x510E527F
	init5 = 0x9B05688C
	init6 = 0x1F83D9AB
	init7 = 0x5BE0CD19
)

// The purpose of this padding is to ensure
// that the padded message is a multiple of 512 bits.
func padding(m []byte) []byte {
	// length of the message m in bits
	l := len(m) * 8
	// number of zero paddding bits needed
	plen := 512 - (l+8+64)%512
	// byte slice of zeros needed
	p := make([]byte, plen/8)
	// add the '1' bit to the end of the message
	m = append(m, 0x80)
	// append the padding bytes
	m = append(m, p...)
	// length of the original message in binary
	mlen := make([]byte, 8)
	binary.BigEndian.PutUint64(mlen, uint64(l))
	// append the length of the original message in bits
	m = append(m, mlen...)
	return m
}

// The message and its padding must be parsed
// into N 512-bit message blocks.
func parsing(m []byte) [][]byte {
	var mb [][]byte
	n := len(m) / chunk
	for i := 0; i < n; i++ {
		mb = append(mb, m[i*chunk:(i+1)*chunk])
	}
	return mb
}

func main() {
	if len(os.Args) != 2 {
		os.Exit(1)
	}
	// message to be hashed
	m := []byte(os.Args[1])

	// preprocessing consists of three steps: padding
	// the message, parsing the message into message
	// blocks, and setting the initial hash value
	m = padding(m)
	mb := parsing(m)
	h := [8]uint32{init0, init1, init2, init3, init4, init5, init6, init7}

  // SHA-256 hash computation 
  // ...

	fmt.Printf("SHA-256 Hash: %x\n", m, mb, h)
}
