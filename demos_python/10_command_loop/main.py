def main() -> None:
    while True:
        command = input("Please enter a command > ")

        if command == "exit":
            break

        print(f"Command: {command}")


if __name__ == "__main__":
    main()
