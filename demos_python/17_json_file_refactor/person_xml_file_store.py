from xml.etree import ElementTree

from person import Person


class PersonXmlFileStore:
    def __init__(self, file: str) -> None:
        self.file = file

    def write(self, people: list[Person]) -> None:
        root = ElementTree.Element("people")
        for person in people:
            person_element = ElementTree.SubElement(root, "person")
            name_element = ElementTree.SubElement(person_element, "name")
            name_element.text = person.name
            age_element = ElementTree.SubElement(person_element, "age")
            age_element.text = str(person.age)
        tree = ElementTree.ElementTree(root)
        tree.write(self.file)

    def __deserialize_person__(self, child: ElementTree.Element) -> Person:
        name_element = child.find("name")
        if name_element is None or name_element.text is None:
            raise ValueError("Invalid XML format")
        age_element = child.find("age")
        if age_element is None or age_element.text is None:
            raise ValueError("Invalid XML format")
        return Person(name_element.text, int(age_element.text))

    def read(self) -> list[Person]:
        tree = ElementTree.parse(self.file)
        root = tree.getroot()
        return [self.__deserialize_person__(child) for child in root]
