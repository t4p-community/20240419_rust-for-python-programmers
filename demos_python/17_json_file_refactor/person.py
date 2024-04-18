class Person:
    def __init__(self, name: str, age: int) -> None:
        self.name = name
        self.age = age

    def __str__(self) -> str:
        return f"Person(name={self.name}, age={self.age})"

    def to_dict(self) -> dict:
        return {"name": self.name, "age": self.age}

    @staticmethod
    def from_dict(data: dict) -> "Person":
        return Person(data["name"], data["age"])
