from functools import reduce


class CartItem:
    def __init__(self, name: str, price: float, quantity: int) -> None:
        self.name = name
        self.price = price
        self.quantity = quantity

    def __str__(self) -> str:
        return (
            f"Item: {self.name}, "
            f"Quantity: {self.quantity}, "
            f"Price: ${self.price:.2f}"
        )


def main() -> None:
    items = [
        CartItem("Apple", 1.99, 2),
        CartItem("Banana", 0.99, 3),
        CartItem("Orange", 2.99, 1),
    ]

    for item in items:
        print(item)

    total = reduce(
        lambda acc, item: acc + item.price * item.quantity, items, 0.0
    )
    print(f"Total: ${total:.2f}")


if __name__ == "__main__":
    main()
