import os
import sys
from systemd import journal
from pyruse import base, config, module, workflow

PYRUSE_ENVVAR = "PYRUSE_EXTRA"
PYRUSE_PATHS = []

def _setPyrusePaths():
    global PYRUSE_ENVVAR, PYRUSE_PATHS
    for p in "/etc/pyruse", os.environ.get(PYRUSE_ENVVAR):
        if p != "" and os.path.isdir(p):
            PYRUSE_PATHS.insert(0, p)
            sys.path.insert(1, p)
    PYRUSE_PATHS.insert(0, os.curdir)

def _doForEachJournalEntry(fct):
    j = journal.Reader(journal.SYSTEM_ONLY)
    j.seek_tail()
    j.get_previous()
    while True:
        event = j.wait(-1)
        if event == journal.APPEND:
            for entry in j:
                fct(j)
    
def main():
    _setPyrusePaths()
    conf = config.Config(PYRUSE_PATHS)
    fback = conf.asMap().get("fallback", {})
    if "all_filters_failed" in fback:
        base.filterFallback = module.get(fback.get("all_filters_failed")).module
    else:
        base.filterFallback = None
    if "finalize_after_last_action" in fback:
        base.actionFallback = module.get(fback.get("finalize_after_last_action")).module
    else:
        base.actionFallback = None
    wf = workflow.Workflow(conf.asMap().get("actions", {}))
    _doForEachJournalEntry(wf.run)

if __name__ == '__main__':
    main()
