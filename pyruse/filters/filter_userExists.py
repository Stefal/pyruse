import pwd
from pyruse import base

class Filter(base.Filter):
    def __init__(self, args):
        super().__init__()
        self.field = args["field"]

    def filter(self, entry):
        try:
            pwd.getpwnam(entry.get(self.field, ""))
            return True
        except KeyError:
            return False
