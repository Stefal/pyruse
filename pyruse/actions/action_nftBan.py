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
        if args is None:
            return # on-boot configuration
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
                    if ban["timestamp"] > 0 and ban["timestamp"] <= now.timestamp():
                        continue
                    elif {k: ban[k] for k in newBan.keys()} == newBan:
                        # should not happen, since the IP is banned…
                        previousTS = ban["timestamp"]
                    else:
                        bans.append(ban)
        except IOError:
            pass # new file

        if previousTS is not None:
            try:
                cmd = list(Action._nft)
                cmd.append("delete element %s {%s}" % (nftSet, ip))
                subprocess.run(cmd)
            except Exception:
                pass # too late: not a problem

        if self.banSeconds:
            until = now + datetime.timedelta(seconds = self.banSeconds)
            newBan["timestamp"] = until.timestamp()
            timeout = self.banSeconds
        else:
            newBan["timestamp"] = 0
            timeout = 0

        self._doBan(timeout, ip, nftSet)
        bans.append(newBan)
        with open(Action._storage, "w") as dataFile:
            json.dump(bans, dataFile)

    def boot(self):
        now = datetime.datetime.utcnow()
        bans = []
        try:
            with open(Action._storage) as dataFile:
                for ban in json.load(dataFile):
                    if ban["timestamp"] == 0:
                        self._doBan(0, ban["IP"], ban["nftSet"])
                        bans.append(ban)
                    elif ban["timestamp"] <= now.timestamp():
                        continue
                    else:
                        until = datetime.datetime.utcfromtimestamp(ban["timestamp"])
                        timeout = (until - now).total_seconds()
                        self._doBan(int(timeout), ban["IP"], ban["nftSet"])
                        bans.append(ban)
        except IOError:
            pass # no file

        with open(Action._storage, "w") as dataFile:
            json.dump(bans, dataFile)

    def _doBan(self, seconds, ip, nftSet):
        if seconds < 0:
            return # can happen when the threshold is crossed while computing the duration
        if seconds == 0:
            timeout = ""
        else:
            timeout = " timeout %ss" % seconds
        cmd = list(Action._nft)
        cmd.append("add element %s {%s%s}" % (nftSet, ip, timeout))
        subprocess.run(cmd)
