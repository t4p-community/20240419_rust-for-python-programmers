from person import Person
from person_store_factory import PersonStoreFactory
from person_store import PersonStore


def main() -> None:
    people = [Person("Alice", 25), Person("Bob", 30), Person("Charlie", 35)]

    try:
        person_store: PersonStore = PersonStoreFactory.create("people.json")
        person_store.write(people)

        people2 = person_store.read()
        for person in people2:
            print(person)

        person_store = PersonStoreFactory.create("people.xml")
        person_store.write(people)

        people2 = person_store.read()
        for person in people2:
            print(person)

    except Exception as e:
        print(f"An error occurred: {e}")


if __name__ == "__main__":
    main()
