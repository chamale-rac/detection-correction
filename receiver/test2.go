package main

import (
	"encoding/csv"
	"fmt"
	"os"
	hamming "receiver/hamming" // Adjust the import path as necessary
	"strconv"
)

func main() {
    // Paths to input and output CSV files
    inputFilePath := "../tests/test_cases_hamming.csv"
    outputFilePath := "../results/results.csv"

    // Open the input CSV file
    inputFile, err := os.Open(inputFilePath)
    if err != nil {
        fmt.Println("Error opening input file:", err)
        return
    }
    defer inputFile.Close()

    // Read the input CSV file
    reader := csv.NewReader(inputFile)
    reader.FieldsPerRecord = -1
    records, err := reader.ReadAll()
    if err != nil {
        fmt.Println("Error reading input file:", err)
        return
    }

    // Open the output CSV file
    outputFile, err := os.Create(outputFilePath)
    if err != nil {
        fmt.Println("Error creating output file:", err)
        return
    }
    defer outputFile.Close()

    // Write the header to the output CSV file
    writer := csv.NewWriter(outputFile)
    writer.Write([]string{"Error Rate", "Length", "Errors", "Precision", "Detected Errors", "Original Message", "Decoded Message"})

    for i, record := range records[1:] { 
        if len(record) != 6 {
            fmt.Printf("Skipping record on line %d: wrong number of fields\n", i+2)
            continue
        }

        rate := record[0] // 1/10, 1/100, 1/1000 per bit
        length := record[1] // 2^1, 2^2 ... 2^10
        original := record[2] // Original message (ASCII)
        // encoded := record[3] // Encoded message (Hamming code)
        noisy := record[4] // Binary message with errors

	    var finalDecodedBinaryMessage string
        var detectedErrors int // Errors the hamming code detected
        var n, m int = 15,11

        for i := 0; i < len(noisy); i += n {
            if i+n > len(noisy) {
                fmt.Println("❌ Error decoding message: Incomplete block")
                fmt.Println("Check you are using the same (n, m) values as the sender.")
                fmt.Println("Also check you are using Hamming code and not other error correction codes.")
                return
            }

            decodedBinaryMessage, err, count := hamming.DecodeHammingCount(noisy[i:i+n], n, m, i)
            if err != nil {
                fmt.Println("❌ Error decoding message:", err)
                return
            }
            detectedErrors += count
            finalDecodedBinaryMessage += decodedBinaryMessage
        }

        var decodedMessage string = decodeMessage(finalDecodedBinaryMessage) // Decoded message (ASCII)

        var errorCount int = countErrors(original, decodedMessage) // we determine how far off the decoded message is from the original message

        var precision float64 = 1 - float64(errorCount) / float64(len(original)) // we determine the precision of the decoded message


        writer.Write([]string{rate, length, fmt.Sprintf("%d", errorCount), fmt.Sprintf("%.4f", precision), fmt.Sprintf("%d", detectedErrors), original, decodedMessage})
    }

    writer.Flush()
    fmt.Println("Results saved to", outputFilePath)
}

func countErrors(original, decoded string) int {    
    errors := 0
    for i := 0; i < len(original) && i < len(decoded); i++ {
        if original[i] != decoded[i] {
            errors++
        }
    }
    // Account for any additional characters in the longer string
    errors += abs(len(original) - len(decoded))

    println(errors)
    return errors
}

// Decode message from binary ASCII
func decodeMessage(binaryMessage string) string {
    var message string
    for i := 0; i < len(binaryMessage); i += 8 {
        end := i + 8
        if end > len(binaryMessage) {
            end = len(binaryMessage)
        }
        binaryChar := binaryMessage[i:end]
        charCode, _ := strconv.ParseInt(binaryChar, 2, 64)
        message += string(charCode)
    }
    return message
}
// Calculate the number of errors between the original and decoded messages
func calculateErrors(original, decoded string) int {
	errors := 0
	for i := 0; i < len(original) && i < len(decoded); i++ {
		if original[i] != decoded[i] {
			errors++
		}
	}
	// Account for any additional characters in the longer string
	errors += abs(len(original) - len(decoded))
	return errors
}

// Absolute value function
func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}
