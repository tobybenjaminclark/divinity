import random
import string


class UniqueNameGenerator:
    def __init__(self, prefix="TEMP"):
        self.prefix = prefix
        self.counter = 0
        self.generated_names = set()

    def generate_name(self):
        # Increment the counter to ensure uniqueness
        self.counter += 1

        # Create a name with a counter value (e.g., TEMP1234)
        unique_name = f"{self.prefix}{self.counter}"

        # If you want even more randomness, use a random string
        # Uncomment the next line to include randomness
        # unique_name = f"{self.prefix}{self.counter}{self._generate_random_string()}"

        # Ensure the name is unique by checking the set
        while unique_name in self.generated_names:
            self.counter += 1
            unique_name = f"{self.prefix}{self.counter}"

        # Add the generated name to the set to track it
        self.generated_names.add(unique_name)

        return unique_name

    def _generate_random_string(self, length=5):
        """Generate a random string of letters and digits"""
        return ''.join(random.choices(string.ascii_uppercase + string.digits, k=length))

