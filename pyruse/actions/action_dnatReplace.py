# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from pyruse import base, dnat

class Action(base.Action, dnat.Matcher):
    def __init__(self, args):
        base.Action.__init__(self)
        sa = args    ["saddrInto"]
        sp = args.get("sportInto", None)
        a  = args.get("addr",  None)
        p  = args.get("port",  None)
        da = args.get("daddr", None)
        dp = args.get("dport", None)
        dnat.Matcher.__init__(self, a, p, da, dp, sa, sp)

    def act(self, entry):
        self.replace(entry)
