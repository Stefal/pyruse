# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import datetime

class Counter():
    _counters = {}

    def _getCounter(self, counterName):
        if counterName not in Counter._counters:
            Counter._counters[counterName] = {}
        return Counter._counters[counterName]

    def _cleanTicks(self, ticks, refDT):
        for tick in list(ticks):
            if tick and tick < refDT:
                ticks.remove(tick)

    def __init__(self, counter):
        self.counterName = counter

    def augment(self, counterKey, duration = None):
        counter = self._getCounter(self.counterName)
        if counterKey not in counter:
            counter[counterKey] = ([], None) # [countUntil,…], graceTermDate
        (ticks, grace) = counter[counterKey]
        now = datetime.datetime.utcnow()
        self._cleanTicks(ticks, now)
        if grace and grace < now:
            grace = None
        if not grace:
            ticks.append(now + duration if duration else None)
            counter[counterKey] = (ticks, grace)
        return len(ticks)

    def lower(self, counterKey):
        counter = self._getCounter(self.counterName)
        if counterKey not in counter:
            return 0
        (ticks, _void) = counter[counterKey]
        now = datetime.datetime.utcnow()
        self._cleanTicks(ticks, now)
        if len(ticks) > 0:
            ticks.pop()
        return len(ticks)

    def reset(self, counterKey, graceDuration = None):
        counter = self._getCounter(self.counterName)
        if graceDuration:
            now = datetime.datetime.utcnow()
            counter[counterKey] = ([], now + graceDuration)
        elif counterKey in counter:
            del(counter[counterKey])
