from color_tool_app import ColorToolApp
from command_loop import CommandLoop


def main() -> None:
    app = ColorToolApp()
    command_loop = CommandLoop(app)
    command_loop.run()


if __name__ == "__main__":
    main()
