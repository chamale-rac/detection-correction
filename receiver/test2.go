package main

import (
	"encoding/csv"
	"fmt"
	"os"
	"strconv"
	crc32 "receiver/crc32" // Adjust the import path as necessary
)

func main() {
    // Paths to input and output CSV files
    inputFilePath := "../tests/test_cases_crc32.csv"
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
    writer.Write([]string{"Error Rate", "Length", "Original", "Encoded", "Noisy", "Has Errors", "Verification Result"})

    // Process each test case
    for i, record := range records[1:] { // Skip the header row
        if len(record) != 6 {
            fmt.Printf("Skipping record on line %d: wrong number of fields\n", i+2)
            continue
        }

        errorRate := record[0]
        length := record[1]
        original := record[2]
        encoded := record[3]
        noisy := record[4]
        hasErrors, _ := strconv.ParseBool(record[5])
		// print type of hasErrors

		fmt.Printf("............................................\n")
		fmt.Printf("Type of hasErrors: %v\n", hasErrors)

        // Verify the noisy message using CRC32
        generator := "100000100110000010001110110110111"
        success, _ := crc32.VerifyCRCAndReturnMessage(noisy, generator)

		fmt.Printf("Success: %v\n", success)

        // Determine the verification result
        verificationResult := "Pass"
		// if it has errors and success is true, we fail
		if hasErrors && success {
			verificationResult = "Fail"
		}
		// if it doesn't have errors and success is false, we fail
		if !hasErrors && !success {
			verificationResult = "Fail"
		}

        // Write the results to the output CSV file
        writer.Write([]string{
            errorRate,
            length,
            original,
            encoded,
            noisy,
            record[5], // Has Errors
            verificationResult,
        })
    }

    writer.Flush()
    fmt.Println("Results saved to", outputFilePath)
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
