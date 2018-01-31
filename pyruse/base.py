# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
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

    def setStepName(self, name):
        self.stepName = name

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
        try:
            nextStep = self.nextStep if self.filter(entry) else self.altStep
        except Exception as e:
            nextStep = self.altStep
            log.error("Error while executing %s: %s." % (type(self), str(e)))
        if nextStep:
            nextStep.run(entry)

class Action(Step):
    def __init__(self):
        super().__init__()

    @abc.abstractmethod
    def act(self, entry):
        pass

    def run(self, entry):
        try:
            self.act(entry)
            nextStep = self.nextStep
        except Exception as e:
            nextStep = None
            log.error("Error while executing %s: %s." % (type(self), str(e)))
        if nextStep:
            nextStep.run(entry)
