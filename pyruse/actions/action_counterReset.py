import datetime
from pyruse import base, counter

class Action(base.Action, counter.Counter):
    def __init__(self, args):
        base.Action.__init__(self)
        counter.Counter.__init__(self, args["counter"])
        self.keyName = args["for"]
        self.save = args.get("save", None)
        graceSeconds = args.get("graceSeconds", None)
        if graceSeconds:
            self.graceSeconds = datetime.timedelta(seconds = graceSeconds)
        else:
            self.graceSeconds = None

    def act(self, entry):
        self.reset(entry[self.keyName], self.graceSeconds)
        if self.save:
            entry[self.save] = 0
