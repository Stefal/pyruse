# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import importlib
from pyruse import log

_modules = {}

class Module:
    def __init__(self, isAction, module, thenRun, elseRun):
        self.isAction = isAction
        self.isFilter = not isAction
        self.module = module
        self.thenRun = thenRun
        self.elseRun = elseRun

def get(moduleDesc):
    if "filter" in moduleDesc:
        isAction = False
        mod = _getModule("pyruse.filters." + moduleDesc["filter"])
        obj = mod.Filter(moduleDesc.get("args", {}))
        thenRun = None
        elseRun = moduleDesc["else"] if "else" in moduleDesc else None
    elif "action" in moduleDesc:
        isAction = True
        mod = _getModule("pyruse.actions." + moduleDesc["action"])
        obj = mod.Action(moduleDesc.get("args", {}))
        thenRun = moduleDesc["then"] if "then" in moduleDesc else None
        elseRun = None
    else:
        raise ValueError("Step is neither “filter” nor “action”: %s\n" % str(moduleDesc))
    return Module(isAction, obj, thenRun, elseRun)

def _getModule(modName):
    if modName not in _modules:
        try:
            module = importlib.import_module(modName)
        except ImportError as e:
            log.error("Module %s not found.\n" % modName)
            raise e
        _modules[modName] = module
    return _modules[modName]
