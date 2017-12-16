# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import abc
from pyruse import log

class Step(abc.ABC):
    def __init__(self):
        self.nextStep = None

    @abc.abstractmethod
    def run(self, entry):
        pass

    def setNextStep(self, obj):
        self.nextStep = obj

class Filter(Step):
    def __init__(self):
        super().__init__()
        self.altStep = None

    def setAltStep(self, obj):
        self.altStep = obj

    @abc.abstractmethod
    def filter(self, entry):
        pass

    def run(self, entry):
        global filterFallback
        try:
            nextStep = self.nextStep if self.filter(entry) else self.altStep
        except Exception as e:
            log.error("Error while executing %s: %s." % (type(self), str(e)))
            nextStep = self.altStep
        if nextStep:
            nextStep.run(entry)
        elif filterFallback is not None:
            filterFallback.run(entry)

class Action(Step):
    def __init__(self):
        super().__init__()

    @abc.abstractmethod
    def act(self, entry):
        pass

    def run(self, entry):
        global actionFallback
        try:
            self.act(entry)
        except Exception as e:
            log.error("Error while executing %s: %s." % (type(self), str(e)))
        if self.nextStep:
            self.nextStep.run(entry)
        elif self != filterFallback and self != actionFallback and actionFallback is not None:
            actionFallback.run(entry)

filterFallback = None
actionFallback = None
