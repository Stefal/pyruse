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

def _doForEachJournalEntry(workflow):
    enc8b = config.Config().asMap().get("8bit-message-encoding", "iso-8859-1")
    j = journal.Reader(journal.SYSTEM_ONLY)
    j.seek_tail()
    j.get_previous()
    while True:
        event = j.wait(None)
        if event == journal.APPEND:
            for entry in j:
                m = entry['MESSAGE']
                if not isinstance(m, str):
                    entry['MESSAGE'] = m.decode(enc8b)
                step = workflow.firstStep
                while step is not None:
                    step = step.run(entry)

def boot(modName):
    _setPyrusePaths()
    conf = config.Config(PYRUSE_PATHS)
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
    _doForEachJournalEntry(wf)

if __name__ == '__main__':
    main()
