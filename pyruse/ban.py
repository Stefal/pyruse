# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import abc
import datetime
import json

class NetfilterBan(abc.ABC):
    def __init__(self, storage):
        self.storage = storage

    def initSelf(self, ipv4Set, ipv6Set, ipField, banSeconds):
        self.ipv4Set = ipv4Set
        self.ipv6Set = ipv6Set
        self.field = ipField
        self.banSeconds = banSeconds

    @abc.abstractmethod
    def setBan(self, nfSet, ip, seconds):
        pass

    @abc.abstractmethod
    def cancelBan(self, nfSet, ip):
        pass

    def act(self, entry):
        ip = entry[self.field]
        nfSet = self.ipv6Set if ":" in ip else self.ipv4Set
        newBan = {"IP": ip, "nfSet": nfSet}

        now = datetime.datetime.utcnow()
        bans = []
        previousTS = None
        try:
            with open(self.storage) as dataFile:
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
                self.cancelBan(nfSet, ip)
            except Exception:
                pass # too late: not a problem

        if self.banSeconds:
            until = now + datetime.timedelta(seconds = self.banSeconds)
            newBan["timestamp"] = until.timestamp()
            timeout = self.banSeconds
        else:
            newBan["timestamp"] = 0
            timeout = 0

        self.setBan(nfSet, ip, timeout)
        bans.append(newBan)
        with open(self.storage, "w") as dataFile:
            json.dump(bans, dataFile)

    def boot(self):
        now = int(datetime.datetime.utcnow().timestamp())
        bans = []
        try:
            with open(self.storage) as dataFile:
                for ban in json.load(dataFile):
                    if ban["timestamp"] == 0:
                        self.setBan(ban["nfSet"], ban["IP"], 0)
                        bans.append(ban)
                    elif int(ban["timestamp"]) <= now:
                        continue
                    else:
                        timeout = int(ban["timestamp"]) - now
                        self.setBan(ban["nfSet"], ban["IP"], timeout)
                        bans.append(ban)
        except IOError:
            pass # no file

        with open(self.storage, "w") as dataFile:
            json.dump(bans, dataFile)
