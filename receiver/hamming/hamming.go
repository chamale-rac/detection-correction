package hamming

import (
	"fmt"
	"math"
	"math/big"
	"strconv"
)

func arrToBinary(arr []int) string {
	binaryStr := ""
	for _, val := range arr {
		binaryStr += strconv.Itoa(val)
	}
	return binaryStr
}

func binaryToDecimal(binary string) int {
	decimal, _ := strconv.ParseInt(binary, 2, 0)
	return int(decimal)
}

// DecodeHamming decodes a Hamming encoded message
func DecodeHamming(encoded string, n, m int) (string, error) {
	r := n - m
	totalBits := len(encoded)
	decodedMessage := make([]rune, m)

	paritiesArr := []int{}

	for i := 0; i < r; i++ { // Go through each redundancy bit
		parityPos := int(math.Pow(2, float64(i))) - 1
		parity := 0
		for j := parityPos; j < totalBits; j += 2 * (parityPos + 1) {
			for k := 0; k < parityPos+1 && j+k < totalBits; k++ { // Going using blocks... 1=[0, 1, 0 ...], 2=[0, 0, 1, 1, 0, 0 ...], 4=[0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0 ...]
				parity ^= int(encoded[j+k] - '0')
			}
		}
		paritiesArr = append(paritiesArr, parity)
	}

	// Using the parities Arr to correct the encoded message
	// Based on the parities Arr, we can determine the position of the error
	// By converting the parities Arr to a binary number, we can determine
	errorBinary := arrToBinary(paritiesArr)
	errorPos := binaryToDecimal(errorBinary)

	if errorPos != 0 {
		fmt.Println("Error detected at position:", errorPos)
		fmt.Println("Correcting the error...")
		encoded = encoded[:errorPos-1] + string(flipBit(rune(encoded[errorPos-1]))) + encoded[errorPos:]
	}

	j := 0
	for i := 1; i <= totalBits; i++ {
		if i&(i-1) != 0 { // Extract only the data bits
			decodedMessage[j] = rune(encoded[i-1])
			j++
		}
	}

	return string(decodedMessage), nil
}

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
