# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import re
from pyruse import base

class Filter(base.Filter):
    def __init__(self, args):
        super().__init__()
        self.field = args["field"]
        reList = []
        for item in args["re"]:
            reList.append(re.compile(item))
        self.reList = reList

    def filter(self, entry):
        for item in self.reList:
            match = item.search(entry.get(self.field, ""))
            if match:
                for name, value in match.groupdict().items():
                    entry[name] = value
                return True
        return False
