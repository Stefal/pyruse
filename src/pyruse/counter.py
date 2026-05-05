# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import datetime

class _GraceAndTicks():
    def __init__(self):
        self.grace = None
        self.ticks = []

class _CounterData():
    def __init__(self):
        self._keyVals = {}

    def clean(self, refDT):
        for k in list(self._keyVals.keys()):
            v = self._keyVals[k]
            if v.grace and v.grace <= refDT:
                v.grace = None
            if v.grace is None:
                # None values (∞) are at the end of the list
                for i in range(0, len(v.ticks)):
                    if v.ticks[i] and v.ticks[i] <= refDT:
                        continue
                    v.ticks = v.ticks[i:]
                    break
                else:
                    del self._keyVals[k]

    def graceActive(self, counterKey, refDT):
        self.clean(refDT)
        return counterKey in self._keyVals and self._keyVals[counterKey].grace

    def augment(self, counterKey, refDT, until):
        self.clean(refDT)

        if counterKey in self._keyVals:
            v = self._keyVals[counterKey]
            if v.grace:
                return 0
        else:
            v = _GraceAndTicks()
            self._keyVals[counterKey] = v
        l = len(v.ticks)

        # chances are that until is greater than the last item
        for i in range(l, 0, -1):
            if until is None or (v.ticks[i - 1] and v.ticks[i - 1] < until):
                v.ticks.insert(i, until)
                break
        else:
            v.ticks.insert(0, until)
        return l + 1

    def lower(self, counterKey, refDT):
        self.clean(refDT)

        v = self._keyVals.get(counterKey, None)
        if v is None or v.grace:
            return 0

        l = len(v.ticks)
        if l == 1:
            del self._keyVals[counterKey]
            return 0

        v.ticks.pop()
        return l - 1

    def reset(self, counterKey, refDT, graceUntil):
        self.clean(refDT)
        if graceUntil:
            v = _GraceAndTicks()
            v.grace = graceUntil
            self._keyVals[counterKey] = v
        elif counterKey in self._keyVals:
            del self._keyVals[counterKey]

class Counter():
    _counters = {}

    def __init__(self, counter):
        if counter not in Counter._counters:
            Counter._counters[counter] = _CounterData()
        self.counter = Counter._counters[counter]

    def augment(self, counterKey, duration = None):
        now = datetime.datetime.utcnow()
        if self.counter.graceActive(counterKey, now):
            return 0
        else:
            return self.counter.augment(counterKey, now, now + duration if duration else None)

    def lower(self, counterKey):
        now = datetime.datetime.utcnow()
        if self.counter.graceActive(counterKey, now):
            return 0
        else:
            return self.counter.lower(counterKey, now)

    def reset(self, counterKey, graceDuration = None):
        now = datetime.datetime.utcnow()
        self.counter.reset(
            counterKey, now,
            now + graceDuration if graceDuration else None
        )
