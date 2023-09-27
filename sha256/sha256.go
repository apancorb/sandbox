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
	size  = 32
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

var k = []uint32{
	0x428a2f98,
	0x71374491,
	0xb5c0fbcf,
	0xe9b5dba5,
	0x3956c25b,
	0x59f111f1,
	0x923f82a4,
	0xab1c5ed5,
	0xd807aa98,
	0x12835b01,
	0x243185be,
	0x550c7dc3,
	0x72be5d74,
	0x80deb1fe,
	0x9bdc06a7,
	0xc19bf174,
	0xe49b69c1,
	0xefbe4786,
	0x0fc19dc6,
	0x240ca1cc,
	0x2de92c6f,
	0x4a7484aa,
	0x5cb0a9dc,
	0x76f988da,
	0x983e5152,
	0xa831c66d,
	0xb00327c8,
	0xbf597fc7,
	0xc6e00bf3,
	0xd5a79147,
	0x06ca6351,
	0x14292967,
	0x27b70a85,
	0x2e1b2138,
	0x4d2c6dfc,
	0x53380d13,
	0x650a7354,
	0x766a0abb,
	0x81c2c92e,
	0x92722c85,
	0xa2bfe8a1,
	0xa81a664b,
	0xc24b8b70,
	0xc76c51a3,
	0xd192e819,
	0xd6990624,
	0xf40e3585,
	0x106aa070,
	0x19a4c116,
	0x1e376c08,
	0x2748774c,
	0x34b0bcb5,
	0x391c0cb3,
	0x4ed8aa4a,
	0x5b9cca4f,
	0x682e6ff3,
	0x748f82ee,
	0x78a5636f,
	0x84c87814,
	0x8cc70208,
	0x90befffa,
	0xa4506ceb,
	0xbef9a3f7,
	0xc67178f2,
}

func rotateRight(x uint32, n int) uint32 {
	return (x >> n) | (x << (32 - n))
}

func rightShift(x uint32, n int) uint32 {
	return x >> n
}

func ch(x, y, z uint32) uint32 {
	return (x & y) ^ (^x & z)
}

func maj(x, y, z uint32) uint32 {
	return (x & y) ^ (x & z) ^ (y & z)
}

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

// Prepares message schedule ("W array") for
// the SHA-256 algorithm dividing each block into
// 16 32-bit words (W[0] through W[15]), and then
// extending these words using a recurrence relation.
func prepareSchedule(bl []byte) []uint32 {
	sch := make([]uint32, 64)
	for i := 0; i < 16; i++ {
		sch[i] = binary.BigEndian.Uint32(bl[i*4 : (i+1)*4])
	}
	for i := 16; i < 64; i++ {
		s0 := rotateRight(sch[i-15], 7) ^ rotateRight(sch[i-15], 18) ^ rightShift(sch[i-15], 3)
		s1 := rotateRight(sch[i-2], 17) ^ rotateRight(sch[i-2], 19) ^ rightShift(sch[i-2], 10)
		sch[i] = s1 + sch[i-7] + s0 + sch[i-16]
	}
	return sch
}

// Responsible for transforming the working variables
// based on the message schedule, thus producing the intermediate
// hash values for each round of computation.
func processSchedule(H [8]uint32, ws []uint32) [8]uint32 {
	// initialize the eight working vars
	// with the (i-1) hash value
	a, b, c, d, e, f, g, h := H[0], H[1], H[2], H[3], H[4], H[5], H[6], H[7]
	// rotation processing
	for t, w := range ws {
		S0 := rotateRight(a, 2) ^ rotateRight(a, 13) ^ rotateRight(a, 22)
		S1 := rotateRight(e, 6) ^ rotateRight(e, 11) ^ rotateRight(e, 25)
		t1 := h + S1 + ch(e, f, g) + k[t] + w
		t2 := S0 + maj(a, b, c)
		// rotate working vars
		h = g
		g = f
		f = e
		e = d + t1
		d = c
		c = b
		b = a
		a = t1 + t2
	}
	// compute the ith intermidate hash value
	H[0] += a
	H[1] += b
	H[2] += c
	H[3] += d
	H[4] += e
	H[5] += f
	H[6] += g
	H[7] += h
	return H
}

// The SHA-256 hash computation which
// results in a 256-bit message digest of
// the message.
func Hash(m []byte) []byte {
	// preprocessing consists of three steps: padding
	// the message, parsing the message into message
	// blocks, and setting the initial hash value
	mpd := padding(m)
	bls := parsing(mpd)
	H := [8]uint32{init0, init1, init2, init3, init4, init5, init6, init7}
  // compute hash
	for _, bl := range bls {
		// prepare the message schedule
		ws := prepareSchedule(bl)
		// process the message schedule
		H = processSchedule(H, ws)
	}
  // convert to byte slice
	hash := make([]byte, size)
	for i, h := range H {
    binary.BigEndian.PutUint32(hash[i*4:i*4+4], h)
	}
	return hash
}

func main() {
	if len(os.Args) != 2 {
		os.Exit(1)
	}
	// message to be hashed
	m := []byte(os.Args[1])
  // SHA-256
	fmt.Printf("%x\n", Hash(m))
}
