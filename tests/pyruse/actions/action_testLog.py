# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from pyruse.actions import action_dailyReport

class Action(action_dailyReport.Action):
    def __init__(self, args):
        super().__init__(args)
        self.filename = args["outFile"]

    def act(self, entry):
        super().act(entry)
        with open(self.filename, "a") as f:
            f.write(str(entry) + "\n")
