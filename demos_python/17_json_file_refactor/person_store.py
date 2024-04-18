from typing import Protocol

from person import Person


class PersonStore(Protocol):
    def write(self, people: list[Person]) -> None: ...

    def read(self) -> list[Person]: ...
