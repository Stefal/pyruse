# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import os
import sys
from systemd import journal
from pyruse import config, module, workflow

PYRUSE_ENVVAR = "PYRUSE_EXTRA"
PYRUSE_PATHS = []

def _setPyrusePaths():
    global PYRUSE_ENVVAR, PYRUSE_PATHS
    for p in "/etc/pyruse", os.environ.get(PYRUSE_ENVVAR):
        if p and os.path.isdir(p):
            PYRUSE_PATHS.insert(0, p)
            sys.path.insert(1, p)
    PYRUSE_PATHS.insert(0, os.curdir)

def _doForEachJournalEntry(fct):
    j = journal.Reader(journal.SYSTEM_ONLY)
    j.seek_tail()
    j.get_previous()
    while True:
        event = j.wait(None)
        if event == journal.APPEND:
            for entry in j:
                fct(entry)

def boot(modName):
    if "action_" in modName:
        module.get({"action": modName}).module.boot()
    elif "filter_" in modName:
        module.get({"filter": modName}).module.boot()
    else:
        raise ValueError("Neither “action_” nor “filter_” found in the module name; the `boot` feature cannot work for %s\n" % modName)

def main():
    _setPyrusePaths()
    conf = config.Config(PYRUSE_PATHS).asMap().get("actions", {})
    wf = workflow.Workflow(conf)
    _doForEachJournalEntry(wf.run)

if __name__ == '__main__':
    main()
