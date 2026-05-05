# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from pyruse import base, dnat

class Action(base.Action, dnat.Mapper):
    def __init__(self, args):
        base.Action.__init__(self)
        sa = (args    ["saddr"],       None                        )
        sp = (args.get("sport", None), None                        )
        a  = (args.get("addr",  None), args.get("addrValue",  None))
        p  = (args.get("port",  None), args.get("portValue",  None))
        da = (args.get("daddr", None), args.get("daddrValue", None))
        dp = (args.get("dport", None), args.get("dportValue", None))
        dnat.Mapper.__init__(self, sa, sp, a, p, da, dp, args.get("keepSeconds", 63))

    def act(self, entry):
        self.map(entry)
