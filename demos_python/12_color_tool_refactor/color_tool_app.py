from color import Color


class ColorToolApp:
    def __init__(self) -> None:
        self.colors: list[Color] = []

    def add_color(self) -> None:
        name = input("Please enter the color name > ")
        hex_code = input("Please enter the color hex code > ")

        color = Color.new(name, hex_code)
        self.colors.append(color)

    def show_colors(self) -> None:
        for color in self.colors:
            color.print()
