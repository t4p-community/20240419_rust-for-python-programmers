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
