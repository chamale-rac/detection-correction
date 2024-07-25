package crc32

// Function to convert a binary string to an integer array
func binaryStringToIntArray(binary string) []int {
	intArray := make([]int, len(binary))
	for i := 0; i < len(binary); i++ {
		if binary[i] == '1' {
			intArray[i] = 1
		} else {
			intArray[i] = 0
		}
	}
	return intArray
}

// Function to convert an integer array to a binary string
func intArrayToBinaryString(arr []int) string {
	binary := ""
	for _, val := range arr {
		if val == 1 {
			binary += "1"
		} else {
			binary += "0"
		}
	}
	return binary
}

// Function to perform modulo-2 division and return the remainder
// The same function we use when encoding
func mod2Division(dividend, divisor []int) []int {
	dividendCopy := make([]int, len(dividend))
	copy(dividendCopy, dividend)

	for i := 0; i < len(dividend)-len(divisor)+1; i++ {
		if dividendCopy[i] == 1 {
			for j := 0; j < len(divisor); j++ {
				dividendCopy[i+j] ^= divisor[j]
			}
		}
	}
	return dividendCopy[len(dividend)-len(divisor)+1:]
}

// Function to verify the received frame using CRC
func VerifyCRC(receivedFrame, generator string) bool {
	frameBits := binaryStringToIntArray(receivedFrame)
	generatorBits := binaryStringToIntArray(generator)

	// Perform modulo-2 division
	remainder := mod2Division(frameBits, generatorBits)

	// Check if remainder is all zeros
	for _, bit := range remainder {
		if bit != 0 {
			return false
		}
	}
	return true
}

func VerifyCRCAndReturnMessage(receivedFrame, generator string) (bool, string) {
    frameBits := binaryStringToIntArray(receivedFrame)
    generatorBits := binaryStringToIntArray(generator)

    // Perform modulo-2 division
    remainder := mod2Division(frameBits, generatorBits)

    // Check if remainder is all zeros
    for _, bit := range remainder {
        if bit != 0 {
            return false, ""
        }
    }

    // Assuming CRC is at the end and its length is len(generator) - 1
    originalMessageBits := frameBits[:len(frameBits)-len(generator)+1]

    // Convert the original message bits back to a binary string
    originalMessage := intArrayToBinaryString(originalMessageBits)

    return true, originalMessage
}

// Every concept get from https://csc-knu.github.io/sys-prog/books/Andrew%20S.%20Tanenbaum%20-%20Computer%20Networks.pdf
// Pages 212-215 Page 212

