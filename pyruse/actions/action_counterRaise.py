import datetime
from pyruse import base, counter

class Action(base.Action, counter.Counter):
    def __init__(self, args):
        base.Action.__init__(self)
        counter.Counter.__init__(self, args["counter"])
        self.keyName = args["for"]
        self.save = args.get("save", None)
        keepSeconds = args.get("keepSeconds", None)
        if keepSeconds:
            self.keepSeconds = datetime.timedelta(seconds = keepSeconds)
        else:
            self.keepSeconds = None

    def act(self, entry):
        count = self.augment(entry[self.keyName], self.keepSeconds)
        if self.save:
            entry[self.save] = count
