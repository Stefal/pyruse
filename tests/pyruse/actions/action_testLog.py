from pyruse.actions import action_dailyReport

class Action(action_dailyReport.Action):
    def __init__(self, args):
        super().__init__(args)
        self.filename = args["outFile"]

    def act(self, entry):
        super().act(entry)
        with open(self.filename, "a") as f:
            f.write(str(entry) + "\n")
