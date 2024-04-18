from color import Color


def main() -> None:
    colors = []

    color1 = Color.new("red", "ff0000")
    colors.append(color1)

    color2 = Color.new("green", "00ff00")
    colors.append(color2)

    color3 = Color.new("blue", "0000ff")
    colors.append(color3)

    for color in colors:
        color.print()


if __name__ == "__main__":
    main()
