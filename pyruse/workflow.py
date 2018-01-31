# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from pyruse import config, log, module

class Workflow:
    def __init__(self, actions):
        self._withDebug = config.Config().asMap().get("debug", False)
        seen = {}
        dangling = []
        firstStep = None
        for label in actions:
            if not label in seen:
                (entryPoint, seen, newDangling) = self._initChain(actions, label, seen)
                if firstStep is None:
                    firstStep = entryPoint
                elif len(dangling) > 0:
                    for setter in dangling:
                        setter(entryPoint)
                dangling = newDangling
        self.firstStep = firstStep

    def _initChain(self, actions, label, seen):
        dangling = []
        previousSetter = None
        firstStep = None
        isPreviousDangling = False
        isThenCalled = False
        for stepNum, step in enumerate(actions[label]):
            if isThenCalled:
                break
            mod = module.get(step)
            obj = mod.module
            if self._withDebug:
                obj.setStepName(label + '[' + str(stepNum) + ']')
            if mod.thenRun:
                (seen, dangling) = \
                    self._branchToChain(obj.setNextStep, mod.thenRun, actions, seen, dangling)
                isThenCalled = True
            if mod.isFilter:
                if mod.elseRun:
                    (seen, dangling) = \
                        self._branchToChain(obj.setAltStep, mod.elseRun, actions, seen, dangling)
                else:
                    dangling.append(obj.setAltStep)
            isPreviousDangling = mod.isFilter and not isThenCalled
            if previousSetter:
                previousSetter(obj)
            else:
                firstStep = obj
            previousSetter = obj.setNextStep
        if isPreviousDangling:
            dangling.append(previousSetter)
        seen[label] = firstStep
        return (firstStep, seen, dangling)

    def _branchToChain(self, parentSetter, branchName, actions, seen, dangling):
        if branchName in seen:
            parentSetter(seen[branchName])
        elif branchName in actions:
            (entryPoint, seen, newDangling) = \
                self._initChain(actions, branchName, seen)
            parentSetter(entryPoint)
            dangling.extend(newDangling)
        else:
            raise ValueError("Action chain not found: %s\n" % branchName)
        return (seen, dangling)
