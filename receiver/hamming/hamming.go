package hamming

import (
	"errors"
	"fmt"
	"math/big"
)

// DecodeHamming decodes a Hamming encoded message
// DecodeHamming decodes a Hamming encoded message
func DecodeHamming(encoded string, n, m, batchPos int) (string, error) {
	// n: total length of the encoded message
	// m: length of the data part
	// batchPos: position in the batch (used to get specific part if encoded is a batch)

	// If the length of encoded message does not match the expected length, return an error
	if len(encoded) != n {
		return "", errors.New("invalid encoded message length")
	}

	// Calculate the number of parity bits (p)
	// Number of parity bits p satisfies the equation: 2^p >= m + p + 1
	p := 0
	for (1 << p) < m+p+1 {
		p++
	}

	// Find the positions of the parity bits
	parityPositions := make([]int, p)
	for i := 0; i < p; i++ {
		parityPositions[i] = (1 << i) - 1
	}

	// Calculate the parity bits from the encoded message
	syndrome := 0
	for i := 0; i < n; i++ {
		if encoded[i] == '1' {
			syndrome ^= (i + 1)
		}
	}

	// If syndrome is not zero, there is an error at the position `syndrome - 1`
	if syndrome > 0 {
		errorPos := syndrome - 1
		if errorPos < n {
			// Print the error position errorPos + batchPos
			fmt.Printf("Error at position %d\n", errorPos+batchPos+1)
			encoded = encoded[:errorPos] + string(flipBit(rune(encoded[errorPos]))) + encoded[errorPos+1:]
		} else {
			return "", errors.New("syndrome indicates an error position outside the encoded message")
		}
	}

	// Extract the original data bits
	originalData := make([]rune, 0, m)
	parityIndex := 0
	for i := 0; i < n; i++ {
		if parityIndex < len(parityPositions) && i == parityPositions[parityIndex] {
			parityIndex++
		} else {
			originalData = append(originalData, rune(encoded[i]))
		}
	}

	return string(originalData), nil
}

// flipBit flips a bit from '0' to '1' or '1' to '0'
func flipBit(bit rune) rune {
	if bit == '0' {
		return '1'
	}
	return '0'
}

// ValidateRedundancyBits checks if 2^r >= n + r + 1
func ValidateRedundancyBits(n, r int) bool {
	nBig := big.NewInt(int64(n))
	rBig := big.NewInt(int64(r))
	two := big.NewInt(2)
	powerOfTwo := new(big.Int).Exp(two, big.NewInt(int64(r)), nil)
	sum := new(big.Int).Add(nBig, rBig)
	sum.Add(sum, big.NewInt(1))

	return powerOfTwo.Cmp(sum) >= 0
}
