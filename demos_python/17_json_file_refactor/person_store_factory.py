from person_store import PersonStore
from person_json_file_store import PersonJsonFileStore
from person_xml_file_store import PersonXmlFileStore


class PersonStoreFactory:
    @staticmethod
    def create(file: str) -> PersonStore:
        if file.endswith(".json"):
            return PersonJsonFileStore(file)
        if file.endswith(".xml"):
            return PersonXmlFileStore(file)
        raise ValueError("Unknown file format")
