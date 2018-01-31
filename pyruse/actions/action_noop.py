# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from pyruse import base

class Action(base.Action):
    def __init__(self, args):
        super().__init__()

    def act(self, entry):
        pass
