# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from pyruse import base

class Filter(base.Filter):
    def __init__(self, args):
        super().__init__()
        self.field = args["field"]
        self.value = args["value"]

    def filter(self, entry):
        return entry.get(self.field, None) == self.value
