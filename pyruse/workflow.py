# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from pyruse import log, module

class Workflow:
    def __init__(self, actions):
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
        self.run = firstStep.run if firstStep else self._noRun

    def _noRun(self, whatever):
        pass

    def _initChain(self, actions, label, seen):
        dangling = []
        previousSetter = None
        firstStep = None
        isPreviousDangling = False
        isThenCalled = False
        for step in actions[label]:
            if isThenCalled:
                break
            mod = module.get(step)
            obj = mod.module
            if mod.isAction:
                if mod.thenRun:
                    (seen, dangling) = \
                        self._branchToChain(obj.setNextStep, mod.thenRun, actions, seen, dangling)
                    isThenCalled = True
                isPreviousDangling = False
            else:
                if mod.elseRun:
                    (seen, dangling) = \
                        self._branchToChain(obj.setAltStep, mod.elseRun, actions, seen, dangling)
                else:
                    dangling.append(obj.setAltStep)
                isPreviousDangling = True
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
