package main

import (
	"bufio"
	"fmt"
	"net"
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
			runHammingListener()
			return
		case "crc32":
			runCRC32Listener()
			return
		default:
			fmt.Printf("Unknown function: %s\n", arg)
			return
		}
	}

	// No command-line argument provided, display the menu
	reader := bufio.NewReader(os.Stdin)

	fmt.Println("--------------------------------------")
	fmt.Println("\t📥 Receiver started...")
	fmt.Println("--------------------------------------")
	fmt.Println("1. Hamming Code")
	fmt.Println("2. CRC32")
	fmt.Print("Choose an option (1 or 2): ")
	choiceStr, _ := reader.ReadString('\n')
	choiceStr = strings.TrimSpace(choiceStr)
	choice, _ := strconv.Atoi(choiceStr)

	switch choice {
	case 1:
		runHammingListener()
	case 2:
		runCRC32Listener()
	default:
		fmt.Println("Invalid choice. Please enter 1 or 2.")
	}
}

func runCRC32Listener() {
	fmt.Println("::: CRC32")

	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Enter the port to listen on: ")
	port, _ := reader.ReadString('\n')
	port = strings.TrimSpace(port)

	listener, err := net.Listen("tcp", ":"+port)
	if err != nil {
		fmt.Println("Error starting listener:", err)
		return
	}
	defer listener.Close()

	fmt.Println("Listening on port", port)

	for {
		conn, err := listener.Accept()
		if err != nil {
			fmt.Println("Error accepting connection:", err)
			continue
		}
		go handleCRC32Connection(conn)
	}
}

func handleCRC32Connection(conn net.Conn) {
	defer conn.Close()

	message, err := bufio.NewReader(conn).ReadString('\n')
	if err != nil {
		fmt.Println("Error reading message:", err)
		return
	}
	message = strings.TrimSpace(message)

	fmt.Println("Received message:\n", message)

	generator := "100000100110000010001110110110111"

	if !validateIsBinary(message) {
		fmt.Println("Invalid input. Please enter a binary string.")
		return
	}

	fmt.Println("⚒️  Verifying CRC32...")

	success, decodedBinaryMessage := crc32.VerifyCRCAndReturnMessage(message, generator)

	if success {
		fmt.Println("- CRC32 verification successful.")
		fmt.Println("- Decoded binary message:\n", decodedBinaryMessage)
		decodedMessage := decodeMessage(decodedBinaryMessage)
		fmt.Println("✅ The original message is:", decodedMessage)
	} else {
		fmt.Println(decodedBinaryMessage)
	}
}

func runHammingListener() {
	fmt.Println("::: Hamming Code")
	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Enter the port to listen on: ")
	port, _ := reader.ReadString('\n')
	port = strings.TrimSpace(port)

	fmt.Print("Number of bits in a block (n): ")
	nStr, _ := reader.ReadString('\n')
	n, _ := strconv.Atoi(strings.TrimSpace(nStr))

	fmt.Print("Number of data bits in a block (m): ")
	mStr, _ := reader.ReadString('\n')
	m, _ := strconv.Atoi(strings.TrimSpace(mStr))

	if !hamming.ValidateRedundancyBits(n, m) {
		fmt.Println("Invalid redundancy bits. m should be equal to 2^n.")
		return
	}

	listener, err := net.Listen("tcp", ":"+port)
	if err != nil {
		fmt.Println("Error starting listener:", err)
		return
	}
	defer listener.Close()

	fmt.Println("Listening on port", port)

	for {
		conn, err := listener.Accept()
		if err != nil {
			fmt.Println("Error accepting connection:", err)
			continue
		}
		go handleHammingConnection(conn, n, m)
	}
}

func handleHammingConnection(conn net.Conn, n, m int) {
	defer conn.Close()

	message, err := bufio.NewReader(conn).ReadString('\n')
	if err != nil {
		fmt.Println("Error reading message:", err)
		return
	}
	message = strings.TrimSpace(message)

	fmt.Println("Received message:\n", message)

	if !validateIsBinary(message) {
		fmt.Println("Invalid input. Please enter a binary string.")
		return
	}

	var finalDecodedBinaryMessage string

	fmt.Println("⚒️  Decoding Hamming code...")

	for i := 0; i < len(message); i += n {
		if i+n > len(message) {
			fmt.Println("❌ Error decoding message: Incomplete block")
			fmt.Println("Check you are using the same (n, m) values as the sender.")
			fmt.Println("Also check you are using Hamming code and not other error correction codes.")
			return
		}

		decodedBinaryMessage, err := hamming.DecodeHamming(message[i:i+n], n, m, i)
		if err != nil {
			fmt.Println("❌ Error decoding message:", err)
			return
		}
		finalDecodedBinaryMessage += decodedBinaryMessage
	}

	fmt.Println("- Decoded binary message:\n", finalDecodedBinaryMessage)
	decodedMessage := decodeMessage(finalDecodedBinaryMessage)
	fmt.Println("✅ The final decoded message is:", decodedMessage)
}

// Decode message from binary ASCII
func decodeMessage(binaryMessage string) string {
	var message string
	for i := 0; i < len(binaryMessage); i += 8 {
		binaryChar := binaryMessage[i : i+8]
		charCode, _ := strconv.ParseInt(binaryChar, 2, 64)
		message += string(charCode)
	}
	return message
}

// Validate if the string is binary
func validateIsBinary(encoded string) bool {
	for _, char := range encoded {
		if char != '0' && char != '1' {
			return false
		}
	}
	return true
}
