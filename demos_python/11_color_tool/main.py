from color import Color


def add_color(colors: list[Color]) -> None:
    name = input("Please enter the color name > ")
    hex_code = input("Please enter the color hex code > ")

    color = Color.new(name, hex_code)
    colors.append(color)


def show_colors(colors: list[Color]) -> None:
    for color in colors:
        color.print()


def main() -> None:
    colors: list[Color] = []

    while True:
        command = input("Please enter a command > ")

        match command:
            case "add":
                add_color(colors)
            case "show":
                show_colors(colors)
            case "exit":
                break
            case _:
                print(f"Unknown command: {command}")


if __name__ == "__main__":
    main()
