def count_and_indicate_errors(binary_message1, binary_message2):
    # Ensure both messages are of the same length
    if len(binary_message1) != len(binary_message2):
        raise ValueError("Binary messages must be of the same length")

    error_positions = []
    for i, (bit1, bit2) in enumerate(zip(binary_message1, binary_message2)):
        if bit1 != bit2:
            error_positions.append(i)

    # Highlight errors in the messages
    highlighted_message1 = ''.join(
        f"[{bit}]" if i in error_positions else bit for i, bit in enumerate(binary_message1)
    )
    highlighted_message2 = ''.join(
        f"[{bit}]" if i in error_positions else bit for i, bit in enumerate(binary_message2)
    )

    # Output the results
    print(f"Binary Message 1: {highlighted_message1}")
    print(f"Binary Message 2: {highlighted_message2}")
    print(f"Number of errors: {len(error_positions)}")
    if error_positions:
        print(f"Error positions: {error_positions}")
    else:
        print("Vemos que el error no fué detectado.")

# Input messages from the user
binary_message1 = input("Enter the first binary message: ")
binary_message2 = input("Enter the second binary message: ")

# Call the function with user inputs
count_and_indicate_errors(binary_message1, binary_message2)
