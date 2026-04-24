# pyruse is intended as a replacement to both fail2ban and epylog
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import string
import subprocess
from pyruse import base, log

class Action(base.Action):
    def __init__(self, args):
        super().__init__()

    def act(self, entry):
        print("We'd like to stop {} - PID {}".format(entry.get("_SYSTEMD_UNIT"), entry.get("_PID")))
        cmd = ("systemctl", "stop", entry.get("_SYSTEMD_UNIT"))
        subprocess.run(cmd)

