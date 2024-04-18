from __future__ import annotations


class Color:
    def __init__(self, name: str, hex_code: str) -> None:
        self.name = name
        self.hex_code = hex_code

    @staticmethod
    def new(name: str, hex_code: str) -> Color:
        return Color(name, hex_code)

    def print(self) -> None:
        print(f"Color: {self.name} (#{self.hex_code})")


def main() -> None:
    # in Python we would not typically use a factory method to create an object
    # but a class method in Python is similar to an associative function in Rust
    color1 = Color.new("red", "ff0000")

    # print is an instance method in Python similar to a method function in Rust
    color1.print()


if __name__ == "__main__":
    main()
