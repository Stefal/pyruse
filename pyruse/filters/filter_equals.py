from pyruse import base

class Filter(base.Filter):
    def __init__(self, args):
        super().__init__()
        self.field = args["field"]
        self.value = args["value"]

    def filter(self, entry):
        return entry.get(self.field, None) == self.value
