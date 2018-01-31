# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import datetime
import json
import os
import subprocess
from pyruse import base, config

class Action(base.Action):
    _storage = config.Config().asMap().get("storage", "/var/lib/pyruse") \
        + "/" + os.path.basename(__file__) + ".json"
    _nft = config.Config().asMap().get("nftBan", {}).get("nft", ["/usr/bin/nft"])

    def __init__(self, args):
        super().__init__()
        self.ipv4Set = args["nftSetIPv4"]
        self.ipv6Set = args["nftSetIPv6"]
        self.field = args["IP"]
        self.banSeconds = args.get("banSeconds", None)

    def act(self, entry):
        ip = entry[self.field]
        nftSet = self.ipv6Set if ":" in ip else self.ipv4Set
        newBan = {"IP": ip, "nftSet": nftSet}

        now = datetime.datetime.utcnow()
        bans = []
        previousTS = None
        try:
            with open(Action._storage) as dataFile:
                for ban in json.load(dataFile):
                    if ban["timestamp"] <= now.timestamp():
                        continue
                    elif {k: ban[k] for k in newBan.keys()} == newBan:
                        # should not happen, since the IP is banned…
                        previousTS = ban["timestamp"]
                    else:
                        bans.append(ban)
        except IOError:
            pass # new file

        if previousTS:
            cmd = list(Action._nft)
            cmd.append("delete element %s {%s}" % (nftSet, ip))
            subprocess.run(cmd)

        until = self._doBan(now, ip, nftSet)

        newBan["timestamp"] = until.timestamp()
        bans.append(newBan)
        with open(Action._storage, "w") as dataFile:
            json.dump(bans, dataFile)

    def boot(self):
        now = datetime.datetime.utcnow()
        bans = []
        try:
            with open(Action._storage) as dataFile:
                for ban in json.load(dataFile):
                    if ban["timestamp"] <= now.timestamp():
                        continue
                    else:
                        bans.append(ban)
                        self._doBan(now, ip, nftSet)
        except IOError:
            pass # no file

        with open(Action._storage, "w") as dataFile:
            json.dump(bans, dataFile)

    def _doBan(self, now, ip, nftSet):
        if self.banSeconds:
            until = now + datetime.timedelta(seconds = self.banSeconds)
            timeout = " timeout %ss" % str(self.banSeconds)
        else:
            until = now + datetime.timedelta(days = 365)
            timeout = ""

        cmd = list(Action._nft)
        cmd.append("add element %s {%s%s}" % (nftSet, ip, timeout))
        subprocess.run(cmd)

        return until
