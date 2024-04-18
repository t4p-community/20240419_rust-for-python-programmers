def num_input(prompt: str) -> int:
    while True:
        try:
            return int(input(prompt))
        except ValueError:
            print("Invalid input. Please enter a number.")
            continue


def main() -> None:
    num1 = num_input("Enter the first number:")
    num2 = num_input("Enter the second number:")
    result = num1 + num2
    print(f"The sum of {num1} and {num2} is {result}")


if __name__ == "__main__":
    main()
