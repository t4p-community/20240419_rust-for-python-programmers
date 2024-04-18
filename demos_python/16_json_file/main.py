import json


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


def main() -> None:
    # Create a list of Person objects
    people = [Person("Alice", 25), Person("Bob", 30), Person("Charlie", 35)]

    # Serialize the list of Person objects to a JSON file
    with open("people.json", "w") as file:
        json.dump([person.to_dict() for person in people], file, indent=2)

    # Deserialize the JSON file to a list of Person objects
    with open("people.json", "r") as file:
        people2 = [Person.from_dict(data) for data in json.load(file)]

    for person in people2:
        print(person)


if __name__ == "__main__":
    main()
