def num_input() -> int:
    try:
        return int(input("Enter a number: "))
    except ValueError as err:
        print(f"Error: {err}")
        exit(-1)


def main() -> None:
    num1 = num_input()
    num2 = num_input()
    result = num1 + num2
    print(f"The sum of {num1} and {num2} is {result}")


if __name__ == "__main__":
    main()
