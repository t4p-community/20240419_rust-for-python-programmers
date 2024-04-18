class Color:
    def __init__(self, name: str, hex_code: str) -> None:
        self.name = name
        self.hex_code = hex_code


def print_color(color: Color) -> None:
    print(f"Color: {color.name} (#{color.hex_code})")


def main() -> None:
    color1 = Color("red", "ff0000")

    print_color(color1)
    # print_color(color1)


if __name__ == "__main__":
    main()
