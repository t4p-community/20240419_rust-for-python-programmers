from color_tool_app import ColorToolApp


class CommandLoop:
    def __init__(self, app: ColorToolApp) -> None:
        self.app = app

    @classmethod
    def command_exit(cls) -> None:
        print("Exiting...")

    @classmethod
    def command_unknown(cls, command: str) -> None:
        print(f"Unknown command: {command}")

    def run(self) -> None:
        while True:
            command = input("Please enter a command > ")

            match command:
                case "add":
                    self.app.add_color()
                case "show":
                    self.app.show_colors()
                case "exit":
                    CommandLoop.command_exit()
                    break
                case _:
                    CommandLoop.command_unknown(command)
