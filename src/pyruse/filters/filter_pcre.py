# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import re
from pyruse import base

class Filter(base.Filter):
    def __init__(self, args):
        super().__init__()
        self.field = args["field"]
        self.re = re.compile(args["re"])
        self.save = args.get("save", [])

    def filter(self, entry):
        match = self.re.search(entry.get(self.field, ""))
        if match:
            for group, name in enumerate(self.save, start = 1):
                entry[name] = match.group(group)
            for name, value in match.groupdict().items():
                entry[name] = value
        return match
