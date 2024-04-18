def console_input(prompt: str, return_type: type = str) -> str | int | float:
    if return_type not in (str, int, float):
        raise ValueError(
            "Invalid return type. Must be one of str, int, float."
        )
    return return_type(input(prompt))


def main() -> None:
    int_value = console_input("Enter an integer: ", int)
    print(f"int_value: {int_value}, int_value type: {type(int_value)}")

    float_value = console_input("Enter a float: ", float)
    print(f"float_value: {float_value}, float_value type: {type(float_value)}")

    str_value = console_input("Enter a string: ")
    print(f"str_value: {str_value}, str_value type: {type(str_value)}")


if __name__ == "__main__":
    main()
