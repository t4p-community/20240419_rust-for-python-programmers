import json
from person import Person


class PersonJsonFileStore:
    def __init__(self, file: str) -> None:
        self.file = file

    def write(self, people: list[Person]) -> None:
        with open(self.file, "w") as file:
            json.dump([person.to_dict() for person in people], file, indent=2)

    def read(self) -> list[Person]:
        with open(self.file, "r") as file:
            return [Person.from_dict(data) for data in json.load(file)]
