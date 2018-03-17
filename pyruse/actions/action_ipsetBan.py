# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import os
import subprocess
from pyruse import ban, base, config

class Action(base.Action, ban.NetfilterBan):
    _storage = config.Config().asMap().get("storage", "/var/lib/pyruse") \
        + "/" + os.path.basename(__file__) + ".json"
    _ipset = config.Config().asMap().get("ipsetBan", {}).get("ipset", ["/usr/bin/ipset", "-exist", "-quiet"])

    def __init__(self, args):
        base.Action.__init__(self)
        ban.NetfilterBan.__init__(self, Action._storage)
        if args is None:
            return # on-boot configuration
        ipv4Set = args["ipSetIPv4"]
        ipv6Set = args["ipSetIPv6"]
        field = args["IP"]
        banSeconds = args.get("banSeconds", None)
        self.initSelf(ipv4Set, ipv6Set, field, banSeconds)

    def act(self, entry):
        ban.NetfilterBan.act(self, entry)

    def setBan(self, nfSet, ip, seconds):
        cmd = list(Action._ipset)
        cmd.extend(["add", nfSet, ip])
        if seconds > 0:
            cmd.extend(["timeout", str(seconds)])
        subprocess.run(cmd)

    def cancelBan(self, nfSet, ip):
        cmd = list(Action._ipset)
        cmd.extend(["del", nfSet, ip])
        subprocess.run(cmd)
