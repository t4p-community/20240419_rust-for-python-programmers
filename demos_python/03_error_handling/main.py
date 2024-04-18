def main() -> None:
    try:
        # type Ctrl+D to raise an EOFError
        name = input("What is your name? ")
        print(f"Hello, {name}!")
    except EOFError:
        print("Handled the EOF Error.")


if __name__ == "__main__":
    main()
