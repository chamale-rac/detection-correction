package main

import (
	"encoding/hex"
	"fmt"
	"log"
)

// CRC-32 polynomial (IEEE 802)
const poly uint32 = 0x04C11DB7

// Function to calculate CRC-32 checksum
func calculateCRC32(data []byte) uint32 {
	crc := ^uint32(0) // Start with all bits set
	for _, b := range data {
		crc ^= uint32(b) << 24
		for i := 0; i < 8; i++ {
			if crc&0x80000000 != 0 {
				crc = (crc << 1) ^ poly
			} else {
				crc <<= 1
			}
		}
	}
	return ^crc // Invert the bits
}

// Function to verify CRC-32 and check message integrity
func verifyCRC32(data []byte, receivedCRC uint32) bool {
	calculatedCRC := calculateCRC32(data)
	return calculatedCRC == receivedCRC
}

// Function to convert a binary string to a byte slice
func binaryStringToBytes(binaryString string) ([]byte, error) {
	length := len(binaryString)
	byteLength := (length + 7) / 8 // Calculate number of bytes needed
	result := make([]byte, byteLength)

	for i := 0; i < length; i++ {
		if binaryString[i] != '0' && binaryString[i] != '1' {
			return nil, fmt.Errorf("invalid character in binary string: %c", binaryString[i])
		}
		if binaryString[i] == '1' {
			result[i/8] |= 1 << (7 - uint(i)%8)
		}
	}
	return result, nil
}

// Function to convert a byte slice to a binary string
func bytesToBinaryString(data []byte) string {
	var binaryString string
	for _, b := range data {
		binaryString += fmt.Sprintf("%08b", b)
	}
	return binaryString
}

// Function to convert a byte slice to an ASCII string
func bytesToASCIIString(data []byte) string {
	return string(data)
}

func main() {
	// Example binary message (for demonstration)
	message := "00110110100100001010011010000110" // Binary for "hello"

	// Convert binary string to byte slice
	messageBytes, err := binaryStringToBytes(message)
	if err != nil {
		log.Fatalf("Error converting binary string to bytes: %v", err)
	}

	// Calculate CRC-32 checksum
	calculatedCRC := calculateCRC32(messageBytes)

	// Simulate received data (message + CRC)
	receivedMessage := append(messageBytes, []byte{
		byte(calculatedCRC >> 24),
		byte(calculatedCRC >> 16),
		byte(calculatedCRC >> 8),
		byte(calculatedCRC),
	}...)

	// Separate message and CRC from received data
	messagePart := receivedMessage[:len(receivedMessage)-4]
	receivedCRCPort := receivedMessage[len(receivedMessage)-4:]
	receivedCRC := uint32(receivedCRCPort[0])<<24 |
		uint32(receivedCRCPort[1])<<16 |
		uint32(receivedCRCPort[2])<<8 |
		uint32(receivedCRCPort[3])

	fmt.Println("Received Message (hex):", hex.EncodeToString(receivedMessage))
	fmt.Println("Original Message (hex):", hex.EncodeToString(messagePart))
	fmt.Printf("Received CRC: 0x%08X\n", receivedCRC)

	// Convert the message bytes back to the original binary string
	originalMessageBinaryString := bytesToBinaryString(messagePart)

	// Verify the message
	if verifyCRC32(messagePart, receivedCRC) {
		fmt.Println("No errors were detected. Original Message in Binary:", originalMessageBinaryString)
		// Convert the message bytes back to ASCII string
		originalASCIIString := bytesToASCIIString(messagePart)
		fmt.Println("Original Message in ASCII:", originalASCIIString)
	} else {
		fmt.Println("Errors were detected. The message is discarded due to errors.")
	}
}
