// Tests the SHA256 hash algorithm as defined
// in FIPS 180-4, using the testig specification.
//
// https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Standards-and-Guidelines/documents/examples/SHA256.pdf
package main

import (
	"bytes"
	"crypto/sha256"
	"testing"
)

func TestSha256(t *testing.T) {
	tests := []string{
		"abc",
		"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
  }

	for _, test := range tests {
    result := Hash([]byte(test))
    expected := expectedHash(test)
    if !bytes.Equal(result, expected) {
			t.Errorf("For %s, expected %x, but got %x", test, expected, result)
		}
	}

}

func expectedHash(m string) []byte {
	// Create a new SHA-256 hash object
	hash := sha256.New()
	// Write the message bytes to the hash
	hash.Write([]byte(m))
	// Get the final hash value as a byte slice
	return hash.Sum(nil)
}
