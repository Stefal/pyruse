# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from datetime import datetime

_mappings = []

def _cleanMappings():
    global _mappings
    now = int(datetime.today().timestamp())
    _mappings = [m for m in _mappings if (now >> m["bits"]) <= m["time"]]

def putMapping(mapping):
    global _mappings
    _cleanMappings()
    _mappings.append(mapping)

def getMappings():
    global _mappings
    _cleanMappings()
    return _mappings

def periodBits(keepSeconds):
    seconds, bits = keepSeconds, 0
    while seconds:
        bits += 1
        seconds = seconds >> 1
    return bits # number of significant bits in keepSeconds

def valueFor(spec, entry):
    return spec[1] if spec[0] is None else entry.get(spec[0], spec[1])

class Mapper():
    def __init__(self, saddr, sport, addr, port, daddr, dport, keepSeconds):
        for spec in [saddr, addr]:
            if spec[0] is None and spec[1] is None:
                raise ValueError("Neither field nor value was specified for address")
        self.saddr = saddr
        self.sport = sport
        self.addr  = addr
        self.port  = port
        self.daddr = daddr
        self.dport = dport
        self.keepBits = periodBits(keepSeconds)

    def map(self, entry):
        saddr = valueFor(self.saddr, entry)
        addr  = valueFor(self.addr,  entry)
        if saddr is None or addr is None:
            return
        sport = valueFor(self.sport, entry)
        port  = valueFor(self.port,  entry)
        daddr = valueFor(self.daddr, entry)
        dport = valueFor(self.dport, entry)
        putMapping(dict(
            bits  = self.keepBits,
            time  = 1 + (int(entry["__REALTIME_TIMESTAMP"].timestamp()) >> self.keepBits),
            saddr = saddr, sport = sport,
            addr  = addr,  port  = port,
            daddr = daddr, dport = dport
        ))

class Matcher():
    def __init__(self, addr, port, daddr, dport, saddr, sport):
        if addr is None and port is None and daddr is None and dport is None:
            raise ValueError("No field was provided on which to do the matching")
        if saddr is None and sport is None:
            raise ValueError("No field was provided in which to store the translated values")
        matchers = []
        updaters = []
        if addr is not None:
            matchers.append((addr, "addr"))
        if port is not None:
            matchers.append((port, "port"))
        if daddr is not None:
            matchers.append((daddr, "daddr"))
        if dport is not None:
            matchers.append((dport, "dport"))
        if saddr is not None:
            updaters.append((saddr, "saddr"))
        if sport is not None:
            updaters.append((sport, "sport"))
        self.matchers = matchers
        self.updaters = updaters

    def replace(self, entry):
        for field, _void in self.matchers:
            if field not in entry:
                return
        for mapping in getMappings():
            for field, mapEntry in self.matchers:
                if entry[field] != mapping[mapEntry]:
                    break
            else:
                for field, mapEntry in self.updaters:
                    entry[field] = mapping[mapEntry]
                return
