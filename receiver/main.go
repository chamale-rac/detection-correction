package main

import (
	"bufio"
	"fmt"
	"os"
	crc32 "receiver/crc32"     // Adjust the import path as necessary
	hamming "receiver/hamming" // Adjust the import path as necessary
	"strconv"
	"strings"
)

func main() {
	// Check for optional command-line argument
	if len(os.Args) > 1 {
		arg := os.Args[1]
		switch arg {
		case "hamming":
			runHamming(bufio.NewReader(os.Stdin))
			return
		case "crc32":
			runCRC32(bufio.NewReader(os.Stdin))
			return
		default:
			fmt.Printf("Unknown function: %s\n", arg)
			return
		}
	}

	// No command-line argument provided, display the menu
	reader := bufio.NewReader(os.Stdin)

	fmt.Println("No function specified. Please choose a function to run:")
	fmt.Println("1. Hamming Code")
	fmt.Println("2. CRC32 (not yet implemented)")
	fmt.Print("Enter your choice (1 or 2): ")
	choiceStr, _ := reader.ReadString('\n')
	choiceStr = strings.TrimSpace(choiceStr)
	choice, _ := strconv.Atoi(choiceStr)

	switch choice {
	case 1:
		runHamming(reader)
	case 2:
		runCRC32(reader)
	default:
		fmt.Println("Invalid choice. Please enter 1 or 2.")
	}
}

func runCRC32(reader *bufio.Reader) {
	fmt.Print("Enter the received message: ")
	receivedFrame, _ := reader.ReadString('\n')
	receivedFrame = strings.TrimSpace(receivedFrame)

	// IEEE 802: x^{32} + x^{26} + x^{23} + x^{22} + x^{16} + x^{12} + x^{11} + x^{10} + x^8 + x^7 + x^5 + x^4 + x^2 + x^1 + 1
	// Page 215 https://csc-knu.github.io/sys-prog/books/Andrew%20S.%20Tanenbaum%20-%20Computer%20Networks.pdf
	generator := "100000100110000010001110110110111"
	// IMPORTANT: CHECK THIS generator MATCHES THE one ENCODER uses

	if !validateIsBinary(receivedFrame) {
		fmt.Println("Invalid input. Please enter a binary string.")
		return
	}

	if crc32.VerifyCRC(receivedFrame, generator) {
		fmt.Println("Frame is correct.")
	} else {
		fmt.Println("Frame is incorrect.")
	}
}

func runHamming(reader *bufio.Reader) {
	fmt.Print(">> (n): ")
	nStr, _ := reader.ReadString('\n')
	n, _ := strconv.Atoi(strings.TrimSpace(nStr))

	fmt.Print(">> (m): ")
	mStr, _ := reader.ReadString('\n')
	m, _ := strconv.Atoi(strings.TrimSpace(mStr))

	if !hamming.ValidateRedundancyBits(n, m) {
		fmt.Println("Invalid redundancy bits. m should be equal to 2n.")
		return
	}

	fmt.Print("Enter the encoded binary message: ")
	encodedMessage, _ := reader.ReadString('\n')
	encodedMessage = strings.TrimSpace(encodedMessage)

	if !validateIsBinary(encodedMessage) {
		fmt.Println("Invalid input. Please enter a binary string.")
		return
	}

	var finalDecodedMessage string // Initialize an empty string to hold the final decoded message

	// Decode in batches of n bits
	for i := 0; i < len(encodedMessage); i += n {
		decodedMessage, err := hamming.DecodeHamming(encodedMessage[i:i+n], n, m, i)
		if err != nil {
			fmt.Println("Error:", err)
			return // Exit if there is an error
		} else {
			finalDecodedMessage += decodedMessage // Concatenate each decoded batch
		}
	}

	// Print the final decoded message after processing all batches
	fmt.Println("The final decoded message is:", finalDecodedMessage)
}

func validateIsBinary(encoded string) bool {
	for _, char := range encoded {
		if char != '0' && char != '1' {
			return false
		}
	}
	return true
}
